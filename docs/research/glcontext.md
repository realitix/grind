# GL Context

Gl context depends on Thread.
It means that one GL context is binded to one thread.
You can't have several context by thread and you can't have several thread
sharing the same GL context.

So We have to create a dict(ThreadId => Context)
