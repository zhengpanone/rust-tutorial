#!/bin/bash
# test.sh

echo "🚀 Testing User Service HTTP API..."

# 测试健康检查
echo -e "\n1. Health Check:"
curl -s http://localhost:8080/health | jq .

# 测试就绪检查
echo -e "\n2. Readiness Check:"
curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/ready
echo ""

# 测试获取用户
echo -e "\n3. Get User:"
for i in {1..3}; do
    curl -s "http://localhost:8080/api/v1/users/$i" | jq -r '.success, .data, .timestamp'
done

# 测试创建用户
echo -e "\n4. Create User:"
curl -s -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"id":"test123"}' | jq .

# 测试服务信息
echo -e "\n5. Service Info:"
curl -s http://localhost:8080/api/v1/info | jq .

# 查看指标
echo -e "\n6. Metrics Preview:"
curl -s http://localhost:9000/metrics | grep -E "^user_service_" | head -20

echo -e "\n✅ All tests completed!"