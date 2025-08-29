# Poor man's pub/sub messaging using IMAP

If you have a web application with the need of a publish-subscribe like messaging, 
but you just have webhosting,
this project is for you!

Chances are good, that your webhost provides also e-mail services with IMAP.
IMAP has a nice feature called *IMAP IDLE*, which allows your email client to receive real-time push notifications for new emails instead of constantly checking the server.

The demo application in this repo (ab)uses this feature to "subscribe" to an e-mail account,
waiting for incomming mails / messages, someone sends ("publish") to it.

## Subscribe

Adapt `main.rs` to your e-mail account data.
Then build the tool using `cargo build`.
Later on you can start it with `cargo run` or by executing the built `imap-pubsub` binary!

## Publish

Just send an e-mail to the configured account, and *imap-pubsub* should receive it within a few seconds.

You can do this for example from within a PHP script on your webhost:


```
<!php
mail(testuser01@example.org, "Message from your Webappa", "A user pressed the publish button");
```

## Test

See [here](test/README.md).
