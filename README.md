Current branch: rudimentary_messaging

Main thread 
- loop for messages from spawned threads
- spawn 2 threads
  - one loops on input read and send it back to main thread
  - one is dedicated to do things and when it finishes a task, sends a message to main thread. 