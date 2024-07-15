Source :- https://github.com/remzi-arpacidusseau/ostep-projects/tree/master/processes-shell

TODO :- Find a way to sync the main thread and other threads. Right now, when running shell commands in parallel, the builtin commands are run on the main execution thread and the remaining commands are run on a separate thread. This means that builtin-cmd & other-cmd / other-cmd & builtin-cmd will execute as :-
main thread -> builtin-cmd,
worker thread -> other-cmd.
