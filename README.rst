==========
RustBridge
==========

What is rustbridge?

  A few people asked what "RustBridge" means. 
  It's an intro to @rustlang workshop for people 
  coming from other programming languages.

  — © `@RustFest <https://twitter.com/RustFest/status/857534247229419521>`_

Getting started
===============

Workshop slides: https://ashleygwilliams.github.io/a-very-brief-intro-to-rust

Setting up a project
--------------------

.. code-block::
   
   # Set up an application project
   # If you want to create a library just avoid 
   # --bin in the command
   cargo init . --bin

You use ``cargo`` for arbitrary tasks:

.. code-block::

   cargo build  # compile app
   cargo test   # run tests on your app
   cargo run    # run your awesome app

Description
===========

This is a simple web application which displays random motivation quotes on every page reload. 

It was developed during the `RustFest <http://2017.rustfest.eu/>`_ workshop held by 
`@ashleygwilliams <https://github.com/ashleygwilliams/>`_ (`@ag_dubs <https://twitter.com/ag_dubs>`_ on Twitter).
The original project can be found at: https://github.com/ashleygwilliams/rust-emergency-compliment.
