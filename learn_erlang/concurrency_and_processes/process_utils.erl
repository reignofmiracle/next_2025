-module(process_utils).
-export([start/0, say_hello/0, identify/0]).

start() ->
    spawn(?MODULE, say_hello, []).

say_hello() ->
    io:format("hello from the process_utils module!~n").

identify() ->
    Pid = self(),
    io:format("My PID is: ~p~n", [Pid]).
