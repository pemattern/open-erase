if [ -z "$STARTED" ]; then
  export STARTED=1
  chmod +x /usr/bin/client-x86_64
  client-x86_64
fi
