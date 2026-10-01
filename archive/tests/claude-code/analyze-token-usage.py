#!/usr/bin/env python3
"""
Analyze token usage from Claude Code session transcripts.
Breaks down usage by main session and individual subagents.
"""

import argparse
import json
import sys
from pathlib import Path
from collections import defaultdict

# Per-million-token pricing: (input, output)
MODEL_PRICING = {
    "opus": (15.0, 75.0),      # Opus 4.6
    "sonnet": (3.0, 15.0),     # Sonnet 4.6 (default)
    "haiku": (0.80, 4.0),      # Haiku 4.5
}

def analyze_main_session(filepath):
    """Analyze a session file and return token usage broken down by agent."""
    main_usage = {
        'input_tokens': 0,
        'output_tokens': 0,
        'cache_creation': 0,
        'cache_read': 0,
        'messages': 0
    }

    # Track usage per subagent
    subagent_usage: dict[str, dict[str, int]] = defaultdict(lambda: {
        'input_tokens': 0,
        'output_tokens': 0,
        'cache_creation': 0,
        'cache_read': 0,
        'messages': 0,
    })
    subagent_descriptions: dict[str, str] = {}

    with open(filepath, 'r') as f:
        for line in f:
            try:
                data = json.loads(line)

                # Main session assistant messages
                if data.get('type') == 'assistant' and 'message' in data:
                    main_usage['messages'] += 1
                    msg_usage = data['message'].get('usage', {})
                    main_usage['input_tokens'] += msg_usage.get('input_tokens', 0)
                    main_usage['output_tokens'] += msg_usage.get('output_tokens', 0)
                    main_usage['cache_creation'] += msg_usage.get('cache_creation_input_tokens', 0)
                    main_usage['cache_read'] += msg_usage.get('cache_read_input_tokens', 0)

                # Subagent tool results
                if data.get('type') == 'user' and 'toolUseResult' in data:
                    result = data['toolUseResult']
                    if 'usage' in result and 'agentId' in result:
                        agent_id = result['agentId']
                        usage = result['usage']

                        # Get description from prompt if available
                        if agent_id not in subagent_descriptions:
                            prompt = result.get('prompt', '')
                            first_line = prompt.split('\n')[0] if prompt else f"agent-{agent_id}"
                            if first_line.startswith('You are '):
                                first_line = first_line[8:]
                            subagent_descriptions[agent_id] = first_line[:60]

                        subagent_usage[agent_id]['messages'] += 1
                        subagent_usage[agent_id]['input_tokens'] += usage.get('input_tokens', 0)
                        subagent_usage[agent_id]['output_tokens'] += usage.get('output_tokens', 0)
                        subagent_usage[agent_id]['cache_creation'] += usage.get('cache_creation_input_tokens', 0)
                        subagent_usage[agent_id]['cache_read'] += usage.get('cache_read_input_tokens', 0)
            except (json.JSONDecodeError, KeyError, TypeError):
                pass

    return main_usage, dict(subagent_usage), subagent_descriptions

def format_tokens(n):
    """Format token count with thousands separators."""
    return f"{n:,}"

def calculate_cost(usage, input_rate=3.0, output_rate=15.0):
    """Calculate estimated cost in dollars with cache discounts."""
    input_cost = usage['input_tokens'] * input_rate / 1_000_000
    cache_write_cost = usage['cache_creation'] * input_rate * 1.25 / 1_000_000
    cache_read_cost = usage['cache_read'] * input_rate * 0.10 / 1_000_000
    output_cost = usage['output_tokens'] * output_rate / 1_000_000
    return input_cost + cache_write_cost + cache_read_cost + output_cost

def main():
    parser = argparse.ArgumentParser(description="Analyze token usage from Claude Code session transcripts.")
    parser.add_argument("session_file", help="Path to session JSONL file")
    parser.add_argument("--model", choices=MODEL_PRICING.keys(), default="sonnet",
                        help="Pricing tier (default: sonnet)")
    args = parser.parse_args()

    if not Path(args.session_file).exists():
        print(f"Error: Session file not found: {args.session_file}")
        sys.exit(1)

    input_rate, output_rate = MODEL_PRICING[args.model]

    # Analyze the session
    main_usage, subagent_usage, subagent_descriptions = analyze_main_session(args.session_file)

    print("=" * 100)
    print(f"TOKEN USAGE ANALYSIS  (pricing: {args.model} — ${input_rate}/${output_rate} per M input/output)")
    print("=" * 100)
    print()

    # Print breakdown
    print("Usage Breakdown:")
    print("-" * 100)
    print(f"{'Agent':<15} {'Description':<35} {'Msgs':>5} {'Input':>10} {'Output':>10} {'Cache':>10} {'Cost':>8}")
    print("-" * 100)

    # Main session
    cost = calculate_cost(main_usage, input_rate, output_rate)
    print(f"{'main':<15} {'Main session (coordinator)':<35} "
          f"{main_usage['messages']:>5} "
          f"{format_tokens(main_usage['input_tokens']):>10} "
          f"{format_tokens(main_usage['output_tokens']):>10} "
          f"{format_tokens(main_usage['cache_read']):>10} "
          f"${cost:>7.2f}")

    # Subagents (sorted by agent ID)
    for agent_id in sorted(subagent_usage.keys()):
        usage = subagent_usage[agent_id]
        cost = calculate_cost(usage, input_rate, output_rate)
        desc = subagent_descriptions.get(agent_id, f"agent-{agent_id}")
        print(f"{agent_id:<15} {desc:<35} "
              f"{usage['messages']:>5} "
              f"{format_tokens(usage['input_tokens']):>10} "
              f"{format_tokens(usage['output_tokens']):>10} "
              f"{format_tokens(usage['cache_read']):>10} "
              f"${cost:>7.2f}")

    # Calculate totals
    total_usage = {
        'input_tokens': main_usage['input_tokens'],
        'output_tokens': main_usage['output_tokens'],
        'cache_creation': main_usage['cache_creation'],
        'cache_read': main_usage['cache_read'],
        'messages': main_usage['messages']
    }

    for usage in subagent_usage.values():
        total_usage['input_tokens'] += usage['input_tokens']
        total_usage['output_tokens'] += usage['output_tokens']
        total_usage['cache_creation'] += usage['cache_creation']
        total_usage['cache_read'] += usage['cache_read']
        total_usage['messages'] += usage['messages']

    total_cost = calculate_cost(total_usage, input_rate, output_rate)

    # Total row in the table
    print("-" * 100)
    print(f"{'TOTAL':<15} {'':<35} "
          f"{total_usage['messages']:>5} "
          f"{format_tokens(total_usage['input_tokens']):>10} "
          f"{format_tokens(total_usage['output_tokens']):>10} "
          f"{format_tokens(total_usage['cache_read']):>10} "
          f"${total_cost:>7.2f}")
    print("=" * 100)

    total_input = total_usage['input_tokens'] + total_usage['cache_creation'] + total_usage['cache_read']
    total_tokens = total_input + total_usage['output_tokens']

    print()
    print("TOTALS:")
    print(f"  Total messages:         {format_tokens(total_usage['messages'])}")
    print(f"  Input tokens:           {format_tokens(total_usage['input_tokens'])}")
    print(f"  Output tokens:          {format_tokens(total_usage['output_tokens'])}")
    print(f"  Cache creation tokens:  {format_tokens(total_usage['cache_creation'])}")
    print(f"  Cache read tokens:      {format_tokens(total_usage['cache_read'])}")
    print()
    print(f"  Total input (incl cache): {format_tokens(total_input)}")
    print(f"  Total tokens:             {format_tokens(total_tokens)}")
    print()
    print(f"  Estimated cost: ${total_cost:.2f}")
    print(f"  (pricing: {args.model} — ${input_rate}/${output_rate} per M, cache read 90% discount, cache write 25% surcharge)")
    print()

if __name__ == '__main__':
    main()
