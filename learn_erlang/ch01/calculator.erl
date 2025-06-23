-module(calculator).
-export([sum_list/1, display_sum/1]).

% Pure function
sum_list([]) -> 0;
sum_list([Head | Tail]) -> Head + sum_list(Tail).

% Impure function
display_sum(List) ->
    Sum = sum_list(List),
    io:format("The sum is ~p~n", [Sum]).
