-module(process_utils2).
-export([start/0, receiver/0]).

start() ->
    Pid = spawn(?MODULE, receiver, []),
    Pid ! {hello, "World"},
    Pid.

receiver() ->
    receive
        {hello, Name} ->
            io:format("Received hello from ~s~n", [Name]);
        _ ->
            io:format("Unknown message received.~n")
    end.
