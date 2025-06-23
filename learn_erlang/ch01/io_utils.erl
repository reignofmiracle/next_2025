-module(io_utils).
-export([print_message/1]).

print_message(Message) ->
    io:format("~s~n", [Message]).
