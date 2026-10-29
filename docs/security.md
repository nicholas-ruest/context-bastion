# Security and deployment boundary

Authentication uses a deterministic operator fixture only in local acceptance; production must replace it with IAM/OIDC. Browser code contains no upstream secret. High-risk effects require an authenticated user event and policy agreement. The test restart endpoint is disabled unless BASTION_TEST_MODE=1. Namespaces prevent fixture leakage. No cloud deployment or spend was authorized. Rollback is a Cloud Run revision rollback after verifying /readyz; persistent RVF segments remain append-only.


