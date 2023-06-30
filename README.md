# guitar

this is for notify any message by sms.

 - read content from file by `-c` argument. default file `read.md`.
 - truncate content by 70 words because of the sms limit words size.
 - to send message in batches.
 - must interval of several seconds while sending message.
 - encrypt the sensitive information.
 - decrypt the sensitive information before sending message.


common cmd options:

 ```bash
 # build
 make build
 make build debug=1
 # send
 make onlyme
 make magic
 make trust
 ```
