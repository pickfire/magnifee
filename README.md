Magnifee (Backend)
==================
LawTech hackathon source code to generate PDF from markdown templates.

Requirements
------------
- pandoc
- rust / cargo

Running
-------

    cargo run
    curl -d 'fullname=John Doe' -d 'address=123 street' \
      -o /tmp/file.output http://127.0.0.1:8080/gen

See Also
--------
[Magnifee Android Frontend](https://github.com/delacrixmorgan/zerocost)
