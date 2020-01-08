# demo2


Local SSL
https://github.com/FiloSottile/mkcert

bench
ab -n 10000 -c 1000 -p post.json -T application/json -H 'Authorization: Token abcd1234' https://127.0.0.1:8080/api/users