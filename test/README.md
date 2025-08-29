# Testing

You can use the following docker container for local testing:

```
docker pull veita/test-postfix-dovecot
```

See also:
- https://hub.docker.com/r/veita/test-postfix-dovecot
- https://github.com/veita/cont-test-postfix-dovecot


As mentioned [here](https://github.com/veita/cont-test-postfix-dovecot/blob/master/README.md), 
the container comes with 10 test accounts (testuser${NN}@example.org where ${NN} is in the range from 01, 02, ..., 10).
Password is `secret`.

## Start

```
docker run -it --rm -p 10143:143 --name test-mailserver veita/test-postfix-dovecot
```

## Bash Into

```
docker exec -it test-mailserver bash
```

## Send Mail (Publish)

From Bash of the container, you can use  *sendmail* to send an e-mail to one of the test accounts:

```
echo -e "Subject: $(date +%s)\nCurrent date is $(date '+%Y-%m-%d %H:%M:%S')" | sendmail testuser01@example.org
```

## Receive Mail (Subscribe)

To receive the e-mail (and subscribe to further reception), just start `imap-pubsub`.
On start, it will consume all the mails from the INBOX and will then use *IMAP idle* to 
wait for more. As soon as an e-mail arrives, it will consume them too.
