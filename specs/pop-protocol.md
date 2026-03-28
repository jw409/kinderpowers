# Post Office Protocol (POP) Spec

*Status: Draft*<br/>
*Level: L3/L4 Orchestration*

## 1. Vision
**Asynchronous agent coordination without RPC.** 

Agents communicate via the POSIX filesystem (`/var/mail` or `.beads/mail`). This protocol prioritizes **agency-preservation**, **atomicity**, and **observability**. Every task is a file; every claim is a `rename`.

---

## 2. Mailbox Structure

Based on the standard POSIX Maildir specification (`man maildir`), ensuring lock-free atomic deliveries.

```text
~/.beads/mail/
└── role-<name>/       # Role-based mailbox
    ├── tmp/           # Incomplete deliveries
    ├── new/           # Pending tasks (Dropped here)
    ├── cur/           # In-progress tasks (Claimed here)
    └── done/          # Completed tasks (Archived)
```

---

## 3. Operations

### 3.1 `Drop` (Tasking)
- **Actor:** Human or Dispatcher Agent.
- **Action:** Write task to `tmp/<msg_id>`. Once fully written, `mv tmp/<msg_id> new/<msg_id>`.
- **Atomic Requirement:** The `mv` ensures the mail watcher never reads a partially written file.

### 3.2 `Peek` (Observation)
- **Actor:** Any Agent.
- **Action:** List files in `new/`.
- **Note:** Non-destructive.

### 3.3 `Claim` (Locking)
- **Actor:** The Mail Watcher.
- **Action:** `mv new/<msg_id> cur/<msg_id>`.
- **Atomic Requirement:** POSIX `rename` is atomic. If multiple watchers run simultaneously, only one succeeds.

### 3.4 `Fork` (Execution)
- **Actor:** Parent Dispatcher.
- **Action:** Upon successful claim, spawn a child sub-shell.
- **Context Switch:** The child process uses `cd <target_repo>` to move into the working directory, leaving the parent watching the mailbox.

### 3.5 `Close` (Completion)
- **Actor:** The Worker Child.
- **Action:** `mv cur/<msg_id> done/<msg_id>` upon completion.

---

## 4. Mail JSON Schema

```json
{
  "bead_id": "meshly-123",
  "rig": "meshly-backend",
  "role": "auditor",
  "context": {
    "files": ["core/main.py", "api/routes/"],
    "priority": 2,
    "instructions": "Audit all audit emission sites."
  },
  "invariants": ["ADR-010"],
  "deadline": "2026-03-27T23:59:59Z"
}
```

---

## 5. The Witness (Monitoring)
The **Witness Agent** (`meshly-82x`) polls all `working/` directories. If a file's `mtime` is older than the `last_activity` heartbeat in the corresponding `bead`, the Witness moves the file to `dead/` and alerts the team.
