Version 1.0.4 (2024.02.04)
-------------------------

* Extended "introspection" that affected 2 commands (`GetExecFromOptuid`, `SetOptuidFromExec`) to "command switches" that are able to NOP (substitute with `Space` implicitly at execution) *any* command. (Those 2 are NOPped by default as before.) The switches are also shown in execution statistics chart.

* `@ random node <bcont>` gives identifier of random node among those whose raw byte content is `bcont`. E.g. `random node 2C` searches among nodes whose content is `PreviousIntegerChannel` command, if such nodes exist.

* Added counts of successfully authenticated (`permitted`) and all (`attempted`) incoming connections from other peers *in the past*, beside the count of *current* (`absorbing`) ones.

* `@ shownode` and `@ statistics` print raw byte representation of `Content` in addition to its name; execution statistics chart includes such representation of selected `Command`.

* Configuration and state of "this" peer are now displayed by just `@ peer` instead of `@ peer info`.

* Added constant sizes of controller-related arrays to ones shown by `@ showsizes`.


Version 1.0.2 (2024.01.23)
--------------------------

* Added bar charts of frequencies of executed `Command`-s, `Construction` instructions, and the ratio of main/alternative `Branch` choices over interval of time preceding current instant. This window appears while `@ run`ning.

![freqs scast gif](https://raw.githubusercontent.com/aelhometta/visuals/main/aelhometta_freqs.gif)

Since the spectacle of running Ælhometta takes most of the time, let it be a little less monotonous.

* Added total counts of `Construction` instructions during `Construct` executions, of main/alternative `Branch` choices and of `Space`-s during ticks, to statistics displayed by `@ stat tick`, which replaced `@ stat comm`.

* Fixed encoding of `IntegerToIntegerChannel`, `IntegerToIntegerIndex` and `ShiftDown`, `ShiftUp` commands accordingly to alphabetical order.


Version 1.0.0 (2024.01.16)
--------------------------

* Initial release, or, Countdown to revealing obvious stupid bugs has started.
