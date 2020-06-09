#!/usr/bin/env bash

# https://superuser.com/questions/1307732/how-to-send-binary-data-in-netcat-to-an-already-established-connection
# Create a temporary fifo.
# The right way to create a temporary file is with mktemp.
# Unfortunately it cannot create fifos.
# It can create a temporary directory though:
tmpd=`mktemp -d`
tmpf="$tmpd"/fifo
mkfifo "$tmpf"
printf "%s\n" "$tmpf"  # just to know the path to the fifo, it may be useful later

# Create a background process that will read the fifo and pass data to a server:
nc 127.0.0.1 7878 < "$tmpf" &
ncpid=$!  # PID may be useful later

# Pay attention if nc doesn't exit prematurely.
# Assuming there's no problem with the connection itself nor the server,
# the above background command will stay connected until you finish
# sending the first bunch of data through the fifo.
# But you want it to stay open and accept multiple writes,
# so open the fifo and don't close it (yet):
exec 3> "$tmpf"

# Now you can send whatever you like through the fifo and the background connection persists:
# echo 'set'     >&3  # sends text
# echo -e '\x80' >&3  # sends "binary"
# cat /etc/issue >&3  # sends file
# cat            >&3  # type whatever you want, terminate with Ctrl+D
echo 'set 1 a'   >&3     # outputs OK
echo 'set 2 b'   >&3     # outputs OK

echo 'get 1'     >&3     # outputs a
echo 'get 2'     >&3     # outputs b
echo 'get 3'     >&3     # outputs nil

echo 'sadd x'    >&3
echo 'sadd x'    >&3
echo 'sadd y'    >&3
echo 'sadd z'    >&3

echo 'srem z'    >&3

echo 'smembers'  >&3     # outputs x\ny

# echo 'debug'     >&3     # outputs entire state: 1a, 2b, x, y

# Assert the output is as expected:
if [[ "$tmpf" =~ "OK\nOK\na\nb\nnil\nx\ny" ]]; then ok; else fail; fi

# After you pass whatever data you need, terminate the nc
# and close the descriptor in the original shell session:
# kill $ncpid
# exec 3>&-

# Remove the temporary directory and its content (i.e. the fifo):
# rm -r "$tmpd"
