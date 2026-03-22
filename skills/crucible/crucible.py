#!/usr/bin/env python3
"""
Crucible: Research Intelligence Framework

12 circuits in 3 tiers:
  READ:   extract, ground, match, converge, project, compose
  LEARN:  decay, calibrate, synthesize
  ATTACK: challenge, replicate, trace

Plus: explode (find weakest hypothesis in each paper)
"""

import argparse
import duckdb
import hashlib
import json
import sys
import httpx
from pathlib import Path
from datetime import datetime, timedelta


DEFAULT_DB = Path("crucible.duckdb")
SCHEMA_SQL = Path(__file__).parent / "schema.sql"


def get_db(db_path: str = None) -> duckdb.DuckDBPyConnection:
    p = Path(db_path) if db_path else DEFAULT_DB
    con = duckdb.connect(str(p))
    if SCHEMA_SQL.exists():
        con.execute(SCHEMA_SQL.read_text())
    return con


def prop_id(text: str) -> str:
    return hashlib.md5(text.encode()).hexdigest()[:12]


# ─── TIER 1: READ ────────────────────────────────────────────────

def cmd_extract(args):
    """Extract propositions from a source (paper, URL, text file)."""
    con = get_db(args.db)
    source = args.source

    # If it looks like an arxiv ID, fetch abstract
    if source.startswith("http") and "arxiv" in source:
        arxiv_id = source.rstrip("/").split("/")[-1]
        resp = httpx.get(f"http://export.arxiv.org/api/query?id_list={arxiv_id}", timeout=30)
        if resp.status_code == 200:
            # Minimal XML parse for abstract + title
            text = resp.text
            title = text.split("<title>")[2].split("</title>")[0].strip() if "<title>" in text else arxiv_id
            abstract = text.split("<summary>")[1].split("</summary>")[0].strip() if "<summary>" in text else ""

            con.execute("""
                INSERT OR REPLACE INTO sources (source_id, domain_id, source_type, title, url, ingested_at)
                VALUES (?, ?, 'paper', ?, ?, now())
            """, [arxiv_id, args.domain, title, source])

            print(f"Source: {arxiv_id} — {title}")
            print(f"Abstract: {abstract[:200]}...")
            print()
            print("Propositions must be extracted by an LLM. Pass this to:")
            print(f"  signpost extract-llm {arxiv_id} --model nemotron_9b")
            return

    # If it's a JSONL file of pre-extracted propositions
    if source.endswith(".jsonl"):
        p = Path(source)
        if not p.exists():
            print(f"File not found: {source}")
            return
        count = 0
        with open(p) as f:
            for line in f:
                data = json.loads(line)
                pid = prop_id(data["text"])
                con.execute("""
                    INSERT OR REPLACE INTO propositions
                    (prop_id, source_id, domain_id, canonical_text, prop_type, extracted_by, extracted_at)
                    VALUES (?, ?, ?, ?, ?, ?, now())
                """, [pid, data.get("source_id", "unknown"), args.domain,
                      data["text"], data.get("type", "claim"), data.get("model", "unknown")])
                count += 1
        print(f"Extracted {count} propositions from {source}")
        return

    print(f"Unknown source format: {source}")
    print("Supported: arxiv URL, .jsonl file")


def cmd_ground(args):
    """Link propositions to Wikipedia for canonical grounding."""
    con = get_db(args.db)
    ungrounded = con.execute("""
        SELECT prop_id, canonical_text FROM propositions
        WHERE wikipedia_url IS NULL
        LIMIT ?
    """, [args.limit]).fetchall()

    print(f"Grounding {len(ungrounded)} propositions...")
    grounded = 0
    for pid, text in ungrounded:
        # Extract likely concept (first noun phrase-ish)
        # Real implementation: use local model to extract concept name
        words = text.split()[:5]
        query = "+".join(words)
        try:
            resp = httpx.get(
                f"https://en.wikipedia.org/w/api.php?action=opensearch&search={query}&limit=1&format=json",
                timeout=10
            )
            if resp.status_code == 200:
                data = resp.json()
                if len(data) >= 4 and data[3]:
                    url = data[3][0]
                    con.execute("UPDATE propositions SET wikipedia_url = ? WHERE prop_id = ?", [url, pid])
                    grounded += 1
                    print(f"  {pid}: {url}")
        except Exception:
            pass

    print(f"Grounded {grounded}/{len(ungrounded)}")


def cmd_converge(args):
    """Cluster propositions into signposts and score convergence."""
    con = get_db(args.db)
    domain = args.domain

    # Count propositions per signpost
    results = con.execute("""
        SELECT s.name, s.convergence_score,
               COUNT(sm.prop_id) as prop_count,
               COUNT(DISTINCT p.source_id) as source_count
        FROM signposts s
        JOIN signpost_members sm ON s.signpost_id = sm.signpost_id
        JOIN propositions p ON sm.prop_id = p.prop_id
        WHERE s.domain_id = ? OR ? IS NULL
        GROUP BY s.name, s.convergence_score
        ORDER BY source_count DESC
    """, [domain, domain]).fetchall()

    print(f"Convergence report for domain: {domain or 'all'}")
    print()
    for name, conv, props, sources in results:
        bar = "#" * sources
        print(f"  [{sources:2d} sources, {props:3d} props] {name}")
        print(f"    {bar}")


def cmd_explode(args):
    """Find the weakest hypothesis in each paper. The one that breaks first."""
    con = get_db(args.db)

    if args.source_id:
        sources = con.execute(
            "SELECT source_id, title FROM sources WHERE source_id = ?",
            [args.source_id]
        ).fetchall()
    else:
        sources = con.execute(
            "SELECT source_id, title FROM sources WHERE domain_id = ? OR ? IS NULL",
            [args.domain, args.domain]
        ).fetchall()

    print(f"Exploding {len(sources)} papers — finding weakest hypothesis in each")
    print()

    for source_id, title in sources:
        props = con.execute("""
            SELECT prop_id, canonical_text, plain_text, tenacity, prop_type
            FROM propositions
            WHERE source_id = ? AND prop_type = 'claim'
            ORDER BY tenacity ASC
            LIMIT 5
        """, [source_id]).fetchall()

        if not props:
            continue

        weakest = props[0]

        # Count how many other propositions cite/depend on this one
        downstream = con.execute("""
            SELECT COUNT(*) FROM cross_links
            WHERE from_prop_id = ? AND link_type IN ('supports', 'extends', 'implements')
        """, [weakest[0]]).fetchone()[0]

        print(f"  {title[:70]}")
        print(f"    WEAKEST: {(weakest[2] or weakest[1])[:90]}")
        print(f"    Tenacity: {weakest[3] or '?'}/1.0 | Blast radius: {downstream} downstream props")

        # Upsert into paper_weakpoints
        con.execute("""
            INSERT OR REPLACE INTO paper_weakpoints
            (source_id, weakest_prop_id, tenacity_score, blast_radius, analyzed_by, analyzed_at)
            VALUES (?, ?, ?, ?, 'signpost-cli', now())
        """, [source_id, weakest[0], weakest[3] or 0.5, downstream])

        print()


def cmd_decay(args):
    """Apply time-based confidence decay to propositions."""
    con = get_db(args.db)

    # Decay based on age and decay_rate
    updated = con.execute("""
        UPDATE propositions
        SET evidence_strength = GREATEST(0.01,
            evidence_strength * POWER(0.5,
                EXTRACT(EPOCH FROM (now() - extracted_at)) / (365.25 * 24 * 3600) / COALESCE(confidence_decay_rate, 0.1)
            )
        )
        WHERE evidence_strength IS NOT NULL
        AND evidence_strength > 0.01
        RETURNING prop_id, evidence_strength
    """).fetchall()

    print(f"Decayed {len(updated)} propositions")

    # Also decay retracted sources to zero
    retracted = con.execute("""
        UPDATE propositions SET evidence_strength = 0.0, tenacity = 0.0
        WHERE source_id IN (SELECT source_id FROM sources WHERE retracted = true)
        AND evidence_strength > 0
        RETURNING prop_id
    """).fetchall()

    if retracted:
        print(f"Zeroed {len(retracted)} propositions from retracted sources")


def cmd_trace(args):
    """Trace evidence chain backward from a proposition."""
    con = get_db(args.db)
    target = args.prop_id

    print(f"Tracing provenance for {target}")
    print()

    # Direct source
    source = con.execute("""
        SELECT p.canonical_text, p.prop_type, p.evidence_strength, p.tenacity,
               p.replication_status, s.title, s.retracted, s.institution
        FROM propositions p
        JOIN sources s ON p.source_id = s.source_id
        WHERE p.prop_id = ?
    """, [target]).fetchone()

    if not source:
        print(f"  Proposition {target} not found")
        return

    print(f"  Proposition: {source[0][:90]}")
    print(f"  Source: {source[5][:70]}")
    print(f"  Retracted: {source[6]} | Institution: {source[7] or '?'}")
    print(f"  Evidence: {source[2] or '?'} | Tenacity: {source[3] or '?'} | Replication: {source[4]}")
    print()

    # What depends on this?
    downstream = con.execute("""
        SELECT cl.link_type, p.canonical_text, p.source_id
        FROM cross_links cl
        JOIN propositions p ON cl.to_prop_id = p.prop_id
        WHERE cl.from_prop_id = ?
    """, [target]).fetchall()

    if downstream:
        print(f"  Downstream ({len(downstream)} props depend on this):")
        for link_type, text, sid in downstream:
            print(f"    [{link_type}] {text[:80]}")

    # What supports this?
    upstream = con.execute("""
        SELECT cl.link_type, p.canonical_text, p.source_id, cl.same_lab
        FROM cross_links cl
        JOIN propositions p ON cl.from_prop_id = p.prop_id
        WHERE cl.to_prop_id = ?
    """, [target]).fetchall()

    if upstream:
        print(f"  Upstream ({len(upstream)} props support this):")
        same_lab_count = sum(1 for u in upstream if u[3])
        if same_lab_count > 1:
            print(f"    ⚠ CITATION RING WARNING: {same_lab_count} supporting props from same lab")
        for link_type, text, sid, same_lab in upstream:
            flag = " [SAME LAB]" if same_lab else ""
            print(f"    [{link_type}] {text[:80]}{flag}")


def cmd_audit(args):
    """Run corpus health checks."""
    con = get_db(args.db)

    total_sources = con.execute("SELECT COUNT(*) FROM sources").fetchone()[0]
    total_props = con.execute("SELECT COUNT(*) FROM propositions").fetchone()[0]
    retracted = con.execute("SELECT COUNT(*) FROM sources WHERE retracted = true").fetchone()[0]
    unreplicated = con.execute("SELECT COUNT(*) FROM propositions WHERE replication_status = 'unreplicated'").fetchone()[0]
    failed_rep = con.execute("SELECT COUNT(*) FROM propositions WHERE replication_status = 'failed'").fetchone()[0]

    # Institution concentration
    inst_dist = con.execute("""
        SELECT institution, COUNT(*) as n FROM sources
        WHERE institution IS NOT NULL
        GROUP BY institution ORDER BY n DESC LIMIT 10
    """).fetchall()

    # Citation ring detection
    rings = con.execute("""
        SELECT COUNT(*) FROM cross_links WHERE same_lab = true
    """).fetchone()[0]

    # Tenacity distribution
    tenacity_dist = con.execute("""
        SELECT
            COUNT(CASE WHEN tenacity < 0.3 THEN 1 END) as fragile,
            COUNT(CASE WHEN tenacity >= 0.3 AND tenacity < 0.7 THEN 1 END) as moderate,
            COUNT(CASE WHEN tenacity >= 0.7 THEN 1 END) as solid,
            COUNT(CASE WHEN tenacity IS NULL THEN 1 END) as unrated
        FROM propositions
    """).fetchone()

    print("=" * 60)
    print("CORPUS HEALTH AUDIT")
    print("=" * 60)
    print(f"  Sources: {total_sources} | Propositions: {total_props}")
    print(f"  Retracted: {retracted} | Replication failures: {failed_rep}")
    print(f"  Unreplicated: {unreplicated}/{total_props} ({100*unreplicated//max(total_props,1)}%)")
    print(f"  Same-lab citations: {rings}")
    print()
    print(f"  Tenacity: {tenacity_dist[0]} fragile, {tenacity_dist[1]} moderate, {tenacity_dist[2]} solid, {tenacity_dist[3]} unrated")
    print()
    if inst_dist:
        print("  Institution concentration:")
        for inst, n in inst_dist:
            pct = 100 * n // total_sources
            print(f"    {inst}: {n} ({pct}%)")
            if pct > 30:
                print(f"    ⚠ >30% from single institution")

    # Compute health score
    health = 1.0
    if total_props > 0:
        health -= 0.2 * (retracted / max(total_sources, 1))
        health -= 0.2 * (failed_rep / total_props)
        health -= 0.1 * (rings / max(total_props, 1))
        if unreplicated / total_props > 0.8:
            health -= 0.1
    health = max(0, min(1, health))

    print(f"\n  Health score: {health:.2f}/1.00")

    # Save audit
    import uuid
    con.execute("""
        INSERT INTO corpus_audits (audit_id, total_sources, total_propositions,
            retraction_count, citation_ring_count, replication_failure_count, corpus_health_score)
        VALUES (?, ?, ?, ?, ?, ?, ?)
    """, [uuid.uuid4().hex[:12], total_sources, total_props, retracted, rings, failed_rep, health])


# ─── CLI ──────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="Crucible: Research Intelligence Framework",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Circuits (12 primitives, 3 tiers):
  READ:   extract, ground, match, converge, project, compose
  LEARN:  decay, calibrate, synthesize
  ATTACK: challenge, replicate, trace

  Plus: explode (find weakest hypothesis per paper)
        audit   (corpus health checks)
        """
    )
    parser.add_argument("--db", default=None, help="DuckDB file path")
    sub = parser.add_subparsers(dest="command")

    # extract
    p = sub.add_parser("extract", help="Extract propositions from a source")
    p.add_argument("source", help="Arxiv URL, JSONL file, or text file")
    p.add_argument("--domain", default="default", help="Domain partition")

    # ground
    p = sub.add_parser("ground", help="Link propositions to Wikipedia")
    p.add_argument("--limit", type=int, default=50)

    # converge
    p = sub.add_parser("converge", help="Show convergence report")
    p.add_argument("--domain", default=None)

    # explode
    p = sub.add_parser("explode", help="Find weakest hypothesis in each paper")
    p.add_argument("--source-id", default=None, help="Specific paper to explode")
    p.add_argument("--domain", default=None)

    # decay
    p = sub.add_parser("decay", help="Apply time-based confidence decay")

    # trace
    p = sub.add_parser("trace", help="Trace evidence chain for a proposition")
    p.add_argument("prop_id", help="Proposition ID to trace")

    # audit
    p = sub.add_parser("audit", help="Run corpus health checks")

    args = parser.parse_args()

    commands = {
        "extract": cmd_extract,
        "ground": cmd_ground,
        "converge": cmd_converge,
        "explode": cmd_explode,
        "decay": cmd_decay,
        "trace": cmd_trace,
        "audit": cmd_audit,
    }

    if args.command in commands:
        commands[args.command](args)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
