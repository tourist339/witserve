CL=$(wc -c test_small.json | tr -s " " | cut -d " " -f 2)
npx loadtest -H "Content-Length:$CL" http://localhost:8080/ -m POST -T application/json -P '{"n":5}' -t 10 -c 3