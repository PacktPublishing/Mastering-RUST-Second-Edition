
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"title":"creativcoder blog","url":"https://creativcoder.github.io"}' \
  127.0.0.1:8080/add

curl 127.0.0.1:8080/links
