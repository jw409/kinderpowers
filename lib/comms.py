import httpx
import subprocess
import shutil
import logging
import time

logger = logging.getLogger(__name__)

def vm_generate(prompt: str, port: int = 8765, model: str = "default", max_tokens: int = 4096, timeout: float = 30.0, json_schema: dict | None = None) -> str | None:
    """Talk to any OpenAI-compatible local server. Returns str or None."""
    body = {"model": model, "messages": [{"role": "user", "content": prompt}], "max_tokens": max_tokens}
    if json_schema:
        body["response_format"] = {"type": "json_schema", "json_schema": json_schema}
    last_exc: Exception | None = None
    for attempt in range(2):
        try:
            r = httpx.post(f"http://127.0.0.1:{port}/v1/chat/completions", json=body, timeout=timeout)
            r.raise_for_status()
            return r.json()["choices"][0]["message"]["content"]
        except (httpx.ConnectError, httpx.TimeoutException) as e:
            last_exc = e
            logger.debug(f"vm_generate [{type(e).__name__}] port {port}: {e}")
            if attempt < 1:
                time.sleep(1)
                continue
        except Exception as e:
            logger.debug(f"vm_generate [{type(e).__name__}] port {port}: {e}")
            return None
    logger.debug(f"vm_generate [{type(last_exc).__name__}] port {port}: retries exhausted")
    return None

def gemini_generate(prompt: str, max_tokens: int = 4096) -> str | None:
    """Gemini CLI (free OAuth). Returns str or None."""
    if not shutil.which("gemini"):
        return None
    try:
        r = subprocess.run(["gemini", "generate", "--max-tokens", str(max_tokens), prompt],
                           capture_output=True, text=True, timeout=60)
        return r.stdout.strip() if r.returncode == 0 else None
    except Exception as e:
        logger.debug(f"gemini_generate failed: {e}")
        return None

def generate(prompt: str, max_tokens: int = 4096, json_schema: dict | None = None) -> str:
    """Cheapest-first: gemini CLI ($0) → GPU ($0) → cmax ($$$)."""
    # 1. Gemini CLI (free) - CLI doesn't support json_schema
    if not json_schema:
        result = gemini_generate(prompt, max_tokens)
        if result:
            return result
    
    # 2. Local GPU
    result = vm_generate(prompt, port=8765, max_tokens=max_tokens, json_schema=json_schema)
    if result:
        return result
        
    # 3. cmax (costs money)
    result = vm_generate(prompt, port=8889, model="claude-sonnet-4-6-20250514", max_tokens=max_tokens, json_schema=json_schema)
    if result:
        return result
        
    raise RuntimeError("All inference backends failed")

def health(port: int = 8765) -> bool:
    """Check if a local VM is alive."""
    try:
        r = httpx.get(f"http://127.0.0.1:{port}/v1/models", timeout=5)
        return r.status_code == 200
    except Exception:
        return False

def discover() -> dict[str, int]:
    """Find all live local inference servers.

    Security note: probes localhost ports without authentication.
    Intended for local dev environments only — do not expose the
    returned port map to untrusted callers or use in production
    without adding auth checks.
    """
    ports = {"gpu": 8765, "cmax": 8889, "ollama": 11434, "lmstudio": 1234, "vllm": 8000}
    return {name: port for name, port in ports.items() if health(port)}
