

  ---
  ```bash
curl -X POST -H "Content-Type: application/json" \
  http://localhost:4771/api/stubs \
  -d '{
    "service": "sentiric.llm.v1.LlamaService",
    "method": "GenerateStream",
    "input": {
      "matches": {
        "user_prompt": ".*"
      }
    },
    "output": {
      "data": {
        "token": "T21uaXNjaWVudCBBcmNoaXRlY3Q6IFNpc3RlbSBDYWxpc2l5b3IhIPCagIA="
      }
    }
  }'
  ```

  ---

  ```bash
docker run --rm --network sentiric-llm-gateway-service_sentiric-net \
  -v $(pwd)/../sentiric-contracts/proto:/proto \
  -v $(pwd)/../sentiric-certificates/certs:/certs \
  fullstorydev/grpcurl \
  -import-path /proto \
  -proto sentiric/llm/v1/gateway.proto \
  -cacert /certs/ca.crt \
  -cert /certs/llm-gateway-service.crt \
  -key /certs/llm-gateway-service.key \
  -d '{"model_selector": "local", "llama_request": {"user_prompt": "Merhaba"}}' \
  llm-gateway-service:16021 \
  sentiric.llm.v1.LlmGatewayService/GenerateDialogStream
  ```