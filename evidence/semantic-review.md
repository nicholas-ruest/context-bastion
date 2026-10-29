# Semantic review pointer

The source-bound semantic review is generated after the immutable candidate is checked out and is stored inside the durable context-bastion-evidence CI artifact. This pointer is not runtime proof. The final repository readback must verify the workflow head SHA, conclusion, artifact digest, manifest, and review verdict.
