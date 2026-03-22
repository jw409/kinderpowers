-- Signpost: Research Intelligence Framework
-- 12 circuits, 7 agents, domain-partitioned proposition graph

-- Domain partitions
CREATE TABLE IF NOT EXISTS domains (
    domain_id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    created_at TIMESTAMP DEFAULT now()
);

-- Source documents
CREATE TABLE IF NOT EXISTS sources (
    source_id VARCHAR PRIMARY KEY,
    domain_id VARCHAR,
    source_type VARCHAR NOT NULL,  -- paper, opinion, market, oss, interview
    title VARCHAR NOT NULL,
    authors VARCHAR[],
    url VARCHAR,
    published_date DATE,
    license VARCHAR,               -- CC-BY, CC-BY-SA, proprietary, unknown
    retracted BOOLEAN DEFAULT false,
    retracted_date DATE,
    institution VARCHAR,           -- for corpus bias tracking
    peer_reviewed BOOLEAN DEFAULT true,
    ingested_at TIMESTAMP DEFAULT now(),
    extraction_model VARCHAR,
    raw_metadata JSON
);

-- Atomic propositions (the core unit)
CREATE TABLE IF NOT EXISTS propositions (
    prop_id VARCHAR PRIMARY KEY,
    source_id VARCHAR,
    domain_id VARCHAR,

    -- Content (9th grade readable)
    canonical_text VARCHAR NOT NULL,
    original_text VARCHAR,         -- verbatim from source
    plain_text VARCHAR,            -- 9th grade rewrite
    prop_type VARCHAR NOT NULL,    -- claim, technique, finding, definition, prediction

    -- Evidence strength
    evidence_strength FLOAT,       -- 0-1
    falsifiable BOOLEAN DEFAULT true,
    resolution_criteria VARCHAR,

    -- Tenacity rating (how fragile is this claim?)
    tenacity FLOAT DEFAULT 0.5,    -- 0=most tenuous, 1=rock solid
    tenacity_reason VARCHAR,       -- why this rating

    -- Replication
    replication_status VARCHAR DEFAULT 'unreplicated',  -- unreplicated, replicated, failed
    replication_count INTEGER DEFAULT 0,

    -- Grounding
    wikipedia_url VARCHAR,

    -- Lifecycle
    confidence_decay_rate FLOAT DEFAULT 0.1,  -- per year
    last_verified TIMESTAMP,
    extracted_by VARCHAR,
    verified_by VARCHAR,
    extracted_at TIMESTAMP DEFAULT now()
);

-- Signposts (convergence clusters)
CREATE TABLE IF NOT EXISTS signposts (
    signpost_id VARCHAR PRIMARY KEY,
    domain_id VARCHAR,
    name VARCHAR NOT NULL,
    description VARCHAR,
    wikipedia_candidate VARCHAR,
    falsifiable_prediction VARCHAR,
    convergence_score FLOAT,
    created_at TIMESTAMP DEFAULT now()
);

-- Signpost membership
CREATE TABLE IF NOT EXISTS signpost_members (
    signpost_id VARCHAR,
    prop_id VARCHAR,
    relevance_score FLOAT DEFAULT 1.0,
    PRIMARY KEY (signpost_id, prop_id)
);

-- Cross-links between propositions
CREATE TABLE IF NOT EXISTS cross_links (
    link_id VARCHAR PRIMARY KEY,
    from_prop_id VARCHAR,
    to_prop_id VARCHAR,
    link_type VARCHAR NOT NULL,    -- supports, contradicts, extends, implements, cites
    confidence FLOAT DEFAULT 0.5,
    same_lab BOOLEAN DEFAULT false, -- citation ring detection
    discovered_by VARCHAR,
    created_at TIMESTAMP DEFAULT now()
);

-- Projections
CREATE TABLE IF NOT EXISTS projections (
    projection_id VARCHAR PRIMARY KEY,
    signpost_id VARCHAR,
    statement VARCHAR NOT NULL,
    confidence FLOAT,
    resolution_date DATE,
    resolved BOOLEAN DEFAULT false,
    resolution_value BOOLEAN,
    market_url VARCHAR,
    market_price FLOAT,
    created_at TIMESTAMP DEFAULT now()
);

-- Paper explosion analysis (most tenuous hypothesis per source)
CREATE TABLE IF NOT EXISTS paper_weakpoints (
    source_id VARCHAR,
    weakest_prop_id VARCHAR,       -- the most tenuous proposition
    tenacity_score FLOAT,          -- how fragile (lower = more fragile)
    attack_vector VARCHAR,         -- what would disprove this?
    blast_radius INTEGER,          -- how many other props depend on this?
    downstream_props VARCHAR[],    -- which props fall if this one falls
    analyzed_by VARCHAR,
    analyzed_at TIMESTAMP DEFAULT now(),
    PRIMARY KEY (source_id)
);

-- Corpus audit trail
CREATE TABLE IF NOT EXISTS corpus_audits (
    audit_id VARCHAR PRIMARY KEY,
    audit_date TIMESTAMP DEFAULT now(),
    total_sources INTEGER,
    total_propositions INTEGER,
    institution_distribution JSON,
    geographic_distribution JSON,
    topic_distribution JSON,
    retraction_count INTEGER,
    citation_ring_count INTEGER,
    replication_failure_count INTEGER,
    corpus_health_score FLOAT      -- 0-1 composite
);
