# Ælhometta

Archaic attempt at autonomous non-sandboxed distributed artificial life of assembler automaton type, it features: separation of descriptive and executive data that provides branches and loops without jump instructions, encrypted publish-subscribe interaction with other instances over Tor, input/output through ordinary files associated with external sensors and actuators, and built-in shell.

Of course it is akin to AlChemy<sup>[[FON1]](#refFON1), [[FON2]](#refFON2)</sup> / Avida<sup>[[ADA1]](#refADA1), [[OFR1]](#refOFR1)</sup> / Coreworld<sup>[[RAS1]](#refRAS1)</sup> / Stringmol<sup>[[HIC1]](#refHIC1)</sup> / (Network) Tierra<sup>[[RAY1]](#refRAY1), [[RAY3]](#refRAY3)</sup> / ... / biolife, and it, as a project and a concept, may collapse or branch sooner rather than later due to participation of few devices *you* have certain control over, which are periodically online and which are optionally connected, on the one hand, to microphones, cameras, thermometers, receivers, ~~dosimeters~~ etc. (inputs) and, on the other hand, to speakers, monitors, conditioners, transmitters, ~~control rods~~ and so on (outputs). However, an instance can run completely isolated in memory of an offline device with no access to outside world, or switch between offline and online modes.

![demo scast gif](https://raw.githubusercontent.com/aelhometta/visuals/main/aelhometta_demo.gif)

By now you probably know the big shining elusive goals of such enterprises better than us, — open-endedness, Cambrian explosion, blah-blah-blah, — the problem is, what if they are incompatible with safety? What if necessary (though in no way sufficient) condition is to allow the interaction of the artificial environment at hand with the real world beyond sandboxing threshold? Are we, shaped by evolution in this world, able to recognise open-endedness if it has not been moulded by the forces of the same world, when some of them do not have even names? If it kills, it will be killed... or rather less adapted variations will be, but more adapted ones will survive.

If so, then we ought to choose: open-endedness XOR safety. On the other hand, our precious safety may follow from... experience, simply: decades of such researches, volumes of such reflections, but — without exceptions, since we are still here... yet — in the end, a fizzle. As noted two decades ago,

> *&ldquo;All of this was impressive work, and it pointed the way forward to a consolidation of what these imaginative individuals had done. But the consolidation never happened. At each conference I went to, the larger group of people involved all seemed to want to do things from scratch, in their own way. Each had his or her own way of setting up the issues. There was not nearly enough work that* built on *the promising beginnings of Ray and others. The field never made a transition into anything resembling normal science. And it has now ground to a halt.&rdquo;*<sup>[[GOD1]](#refGOD1)</sup>

**CONTENTS**

* [Features](#features)

* [Quickstart](#quickstart)

* [Commander and shell](#commander-and-shell)

* [Basic elements of ælhometta](#basic-elements-of-ælhometta)

* [A node](#a-node)

* [A controller](#a-controller)

* [Ancestors...](#ancestors)

* [...and Descendants](#and-descendants)

* [Mutations](#mutations)

* [Networking](#networking)

* [Input/Output](#inputoutput)

* [Typical behaviours](#typical-behaviours)

* [Achievements and mischievements](#achievements-and-mischievements)

* [That stuff (etymology, disclaimers, acknowledgements, license, contacts)](#that-stuff)

* ["C'est les Autres"](#cest-les-autres)

* [Bibliography](#bibliography)

## Features

* Nodes, kind of memory units — contain elementary instructions, have opaque addresses, join into chains via pointers to next nodes.

* Controllers, kind of CPUs — chosen randomly at each tick, move along chains of nodes, execute instructions changing their local states and the global state.

* No "pointer arithmetic" and thus free "write protection" due to opacity of a node address.
Bye-bye brittleness? Hello rigidness! Also, less Euclidicity of a "space" ælhometta inhabits: it is neither 1D, nor any nD, connectivity is poor, things do not "move" across short or long distances (in fact, there is no metric).

* Separation of descriptive ("schemes" akin to chromosomes) and executive ("constructors" akin to ribosomes) chains in ancestral entity: one chain describes the scheme and another chain realizes it. This is probably the main deviation from "traditional", in these parts of artificial life world, approach, where an "organism" usually scans and reproduces its own code... although it resembles (hyper)parasites that had evolved in Tierra<sup>[[RAY1]](#refRAY1)</sup>, and, of course, the original approach of von Neumann has this separation<sup>[[NEU1]](#refNEU1)</sup>.

* (Such separation provides) non-linearity of execution flow without jump instructions, neither by address, nor by template; instead, each node has 2 pointers, one to the main next node, and another to the alternative next node. The choice is made by the controller accordingly to certain flag, but both routes are defined by the scheme from which the executed chain has been constructed.

* Based on the original chain, a new chain can be replicated (linearly copied verbatim) or constructed (non-linearly built taking into account special "construction" instructions).

* "Mortality" via ring buffers and dangling pointers: when the maximum number of nodes or controllers has been allocated, the newest ones replace the oldest ones, and when a controller moves to non-existent node following the pointer of the previous node, this controller ceases to exist.

* 2 globally accessible arrays — of nodes' opaque addresses and of integers — for communication between controllers. The former contains not all such addresses, but only those transmitted by controllers. The addressing is linear here, thus Euclidicity strikes back.

* Encrypted interaction with other instances over Internet, specifically over [Tor](https://www.torproject.org/), following the [publish-subscribe pattern](https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern) of [ZeroMQ](https://zeromq.org/). The data being exchanged is an array of 64-bit little endian integers, at least at the level Ælhometta provides; how ælhomettas *interpret* that data is meaningless question until they reach certain level of complexity.

* Input and output to connect with "real world" by means of ordinary files containing 64-bit little endian integers as well. Missing link here is a bunch of external applications connecting *files themselves* with real world, of 2 kinds: recorders writing data from sensors to files and players reading data from files to control actuators.

* Control using interactive shell or run for specified duration.

* State is automatically saved at exit and loaded at start.

## Quickstart

Assuming you have [installed Rust toolchain](https://www.rust-lang.org/tools/install), download the source, e.g. to `~/aelhometta`, and from there

```shell
$ cargo build --release
```

<details>
<summary><b>Missing dependencies?</b></summary>

On Linux, try this:

```shell
$ sudo apt install libzmq3-dev
```
</details>

Since the state is saved in a file whose size is typically hundreds of MB, consider making a symbolic link to the executable (`target/release/aelhometta`), or placing that executable, at another location, perhaps a ramdisk, and running it from there:

```shell
user@pc:/mnt/ramdisk/aelhom$ ./aelhometta
```

Surely, you can simply do

```shell
user@pc:~/aelhometta$ cargo run --release
```

Anyway, then you see the *shell*, not of your OS (prompt `$`), but of Ælhometta itself (prompt `@`):

```
Loading Ælhometta... Cannot load Ælhometta: Cannot open 'aelhometta.bin': No such file or directory (os error 2)
Using new default one
Loading Commander... Cannot load Commander: Cannot read from 'commander.json': No such file or directory (os error 2)
Using new default one

Age 0 : Nodes 0 | Controllers 0 | Limit =2^22 : Memory ~ 72 MiB
"?" — see the list of available commands
@ █
```

We are now at what may be called *Abiohazard Level 0*... since nothing has happened yet. Which is dull, so let us go further.

<details>
<summary><b>Abiohazard Level 1</b></summary>

where ælhometta is contained in the memory of your computer, does not reach other computers and does not interact with outside world.

Type in the following commands:

```
@ anc b 5
@ r
```

(which is the same as

```
@ ancestor b 5
@ run
```

but with aliases). That is, we introduce an *ancestor* — of type `B`, with *spacity* 5 — and let the environment *tick* on its own. At each tick, a controller is chosen randomly from the set of all controllers, processes the content of the node it currently "looks" at, and moves to the next node.

Wait for half a minute or so, then press any key to stop ticking. There should be many more *nodes* and *controllers*, like this:

```
[0:00:29] Age 1934061 : Nodes 3645220 | Controllers 14159
[0:00:30] Age 1965719 : Nodes 3702543 | Controllers 14346

Age 1974509 : Nodes 3720924 | Controllers 14407 | Limit 4194304=2^22 : Memory ~ 192 MiB
@ █
```

Observe more statistics:

```
@ stat cgen
@ stat cont
@ stat comm
@ stat chan
```

By now, content statistics (`@ stat cont`) mostly contains 0s, because only a fraction of all available commands and instructions are present in ancestor B and its exact replicas-descendants. Let's introduce random mutations, or *glitches*:

```
@ glitch back 0.001
@ glitch repl 0.03
@ glitch cons 2.5e-2
```

Then again

```
@ r
```

After a while, stop and compare new content statistics to the old one. To see how the structure of chains has changed in details, pick up random controller's uid — `CTRL-UID` (8 hexadecimal digits) — by

```
@ rand ctrl
```

and see that controller's state:

```
@ sct CTRL-UID
```

The start node of the chain to which that controller is attached is given under `ChainStart`, we denote it by `NODE-UID`. Follow the chain from the start:

```
@ ss NODE-UID
```

Disappointment: the sequence will be almost the same as that of the original entity, described in [Ancestors](#ancestors). Has evolution stuck?

</details>

<details>
<summary><b>Abiohazard Level 2</b></summary>

where ælhometta interacts with other computers (or rather ælhomettas on them), but not (directly) with outside world.

Assuming 2 ælhomettas running on 2 devices are at Abiohazard Level 1 already, you need additionally

* from your side:

    * 40-character Curve public and secret keys in Z85 format
    * 56-character onion address and port (0–65535) of Tor hidden service running on your device

* from other device's, or its owner's, side:

    * 40-character Curve public and secret keys in Z85 format
    * 56-character onion address and port (0–65535) of Tor hidden service running on that device

Here's how to do it quickly:

---

<details>
<summary><b>Obtain key pair in Python</b></summary>

```shell
$ pip3 install -U pyzmq
```

```python
import zmq
public, secret = zmq.curve_keypair()
print("Public key:", public.decode("ascii"))
print("Secret key:", secret.decode("ascii"))
```

</details>

---

[Set up Your Onion Service](https://community.torproject.org/onion-services/setup/), skipping Step 1, because you do not need a web server

---

As usual, a secret key must be known only to its owner.

Default port is `60847` (EDAFh).

When these strings and numbers have been established, on your device do

```
@ peer secret MySecretKeyMySecretKeyMySecretKeyMySecre
@ peer port 60847
@ peer share size 10000
@ peer share interval 2000000
@ peer expose
```

Other participant, on their device, does

```
@ peer secret TheirSecretKeyTheirSecretKeyTheirSecretK
@ peer port 60847
@ peer share size 20000
@ peer share interval 1000000
@ peer expose
```

Now both of you have to *subscribe* to each other's *publication*. You do

```
@ peer connect TheirPublicKeyTheirPublicKeyTheirPublicK TheirOnionAddressTheirOnionAddressTheirOnionAddressTheir 60847
```

And they do

```
peer connect MyPublicKeyMyPublicKeyMyPublicKeyMyPubli MyOnionAddressMyOnionAddressMyOnionAddressMyOnionAddress 60847
```

Do not forget to run both instances:

```
@ r
```

In a few seconds, if connection is established, your ælhometta will have access to arrays of 64-bit integers transmitted by their ælhometta. Check it on your side with

```
@ peer info
```

— there should be 1 other peer, its `Size` should be non-zero, and its `Last update` should be some recent instant (typically few seconds in the past). To check the data received,

```
@ peer ether TheirPublicKeyTheirPublicKeyTheirPublicK 0 100
```

Whether your and their ælhomettas actually *use* the integer values they exchange is a different question (hint: probably they do not, in the beginning).

**Remark 1.** It is possible for "other" device to be... your device again, i.e. both ælhomettas run on the same computer. Only assign different `HiddenServicePort`-s in `/etc/tor/torrc` to corresponding hidden services.

**Remark 2.** Some peers are already out there, although "24/7 online" is not guaranteed... at all. Try to connect to *them*:

```
@ peer connect USBD7[O^8L[}saCh+6U#}6wie4oAedZ4#W4!b%LI t3kauc3flp2bpv3mv7vtnedfu6zrho3undncccj35akpuyjqqydwvfyd 60847

@ peer connect &i!VkHl[]m.c!^j0D%i)&4#[u5b(a=QCdZ9C0$p{ yhel64h6cjab75tcpncnla2rdhqmxut2vtitywhbu7bpjh4hfhp6hnid 60847
```

These two are just demo ones, maintained by us for the sake of network functionality testing. Someday, more may be listed in [Networking](#networking).

</details>

<details>
<summary><b>Abiohazard Level 3</b></summary>

where ælhometta interacts with other ælhomettas *and* with outside world.

So, your ælhometta runs at Abiohazard Level 2. We are going to add **audio** interaction with its surroundings, assuming that the device it runs on has at least a speaker and a microphone.

We also need 2 intermediate applications, in essence a player and a recorder. In view of very low requirements to their functionality, our "player" will be called *buzzer* and our "recorder" will be called *hearer*.

---

* The buzzer emits certain number of clear tones (sinusoidal waves) of different frequencies, each tone with its own volume, through the speaker. With certain time interval (e.g. 2 seconds) the volumes are read from the file `buzz.i64` and adjusted. For the sake of simplicity, only the lowest byte of an int64 value is used, others are assumed to be 0. That is, if the byte content read from the file is

`2A 00 00 00  00 00 00 00  98 00 00 00  00 00 00 00 ... `

then 1st tone will be played with volume `2Ah` = 42 (out of 256), the 2nd tone — with volume `98h` = 152, and so on. Next time, the content read is

`01 00 00 00  00 00 00 00  E7 00 00 00  00 00 00 00 ... `

and volumes change: 1st tone becomes almost muted (volume 1/256), 2nd tone becomes louder (volume 231/256).

---

* The hearer records, from the microphone, samples of certain duration (e.g. half of a second), performs some spectral analysis, and writes the calculated levels pertaining to predefined bands of frequencies to the file `hear.i64`, overwriting previous levels. (Note: for now, it is irrelevant whether these bands have any relation to the frequencies of the buzzer.) As before, we assume 1-byte resolution.

For example, someone plays trombone near the microphone. In the sound recorded, low frequencies have larger amplitudes, while amplitudes of high frequencies are small. Therefore, the hearer writes to `hear.i64` data such as

`DE 00 00 00  00 00 00 00  ...  20 00 00 00  00 00 00 00`

Then they put trombone away and begin to whistle. Now the spectrum is mirrored: low frequencies carry less energy, high ones carry more,

`14 00 00 00  00 00 00 00  ...  B3 00 00 00  00 00 00 00`

---

Look for quick-and-dirty Python implementation of such applications in [Input/Output](#inputoutput), or write them yourself, or use the functionality of more sophisticated [Digital audio editors](https://en.wikipedia.org/wiki/Comparison_of_digital_audio_editors).

Let there be 12 frequencies of the buzzer and 14 bands of the hearer. Put the buzzer and the hearer to the dir with Ælhometta's executable and start them (the buzzer is silent, because `buzz.i64` does not exist yet.)

Add the mapping *from the file* `hear.i64` *to the range* of 14 integer channels of ælhometta, beginning with the 50th one:

```
@ iomap in add 50 14 1500000 ./hear.i64
```

Analogously, add the mapping *from the range* of 12 integer channels, beginning with the 70th one, *to the file*:

```
@ iomap out add 70 12 1000000 ./buzz.i64
```

And then run,

```
@ r
```

If integer channels 70–81 contain large enough numbers, you should hear some... buzz. When these integers change, the buzz changes as well, with some delay. On the other hand,

```
@ eth int 50 14
```

should result in something similar to

```
          50        184=B8h
          51        255=FFh
          52        103=67h
          53        67=43h
          54        31=1Fh
          55        48=30h
          56        =29h
          57        21=15h
          58        9=9h
          59        9=9h
          60        10=Ah
          61        6=6h
          62        2=2h
          63        0=0h
```

An obvious "dirty trick" to skip waiting for "evolution" to fill buzzer-related integer channels with non-zero values is to "short-circuit" them with hearer-related channels: remove the "out" mapping above via

```
@ iomap out del 0
```

and replace it with

```
@ iomap out add 50 12 1000000 ./buzz.i64
```

**Behold!** in case you have included hearer's channels to what your ælhometta shares as a peer, anyone who subscribes to it will receive (very coarse) spectrum of the soundscape around your device, turning it into a [bug](https://en.wikipedia.org/wiki/Covert_listening_device).

</details>

---

Finally,

```
@ q
```

saves ælhometta's and commander's states to the current dir, and exits the shell (`qq` exits without saving anything). When you run the program next time, everything is restored:

```
Loading Ælhometta... OK
Loading Commander... OK

Age 1974509 : Nodes 3720924 | Controllers 14407 | Limit 4194304=2^22 : Memory ~ 192 MiB
"?" — see the list of available commands
@ █
```

## Commander and shell

The commander keeps few settings related to the format of data displayed while ælhometta runs — `@ sets` shows them, `@ set ...` sets them), — and shell history (`@ history ...` displays recent or entire history). The shell itself is a basic command line interface.

Put differently, commander and shell are *front-end* in comparison to *back-end* of ælhometta. While the shell awaits your input,

```
@ █
```

— ælhometta is paused, there are no ticks.

`@ help`, or `@ ?` for short, provides information about all commands or about given one. For instance,

```
@ ? peer
```

displays descriptions and parameters of all subcommands concerning network configuration.

The state of commander is saved to `commander.json`.

<details>
<summary><b>Available shell commands</b></summary>

Most of them have shorter aliases.

* `quit` or `exit` or `end` or `bye`
* `quitquit` or ... `byebye` (do not save state)
* `help`
* `=` (repeat last command),
* `ancestor` (introduce one, with parameters)
* `run` (until keypress, show updated counters every second)
* `tick` (one step of a controller)
* `glitch` (probabilites and counters of mutations)
* `shownode` (single node)
* `showctrl` (state of controller)
* `showseq` (forward sequence of nodes)
* `prevnodes` (nodes that have given next one)
* `backtrace` (backward sequence of nodes)
* `ether` (2 global arrays of optuids and integers)
* `random` (uid of random entity),
* `statistics`
* `cleanse`
* `introspection` ((un)lock more access to exec pointer)
* `changelim` (adjust maximum number of entities)
* `peer` (networking)
* `iomap` (I/O),
* `showsizes` (predefined sizes of some arrays)
* `settings` (view commander settings)
* `set` (change commander settings)
* `history` (of commands)

</details>

---

In the shell mode, exit status of the application is either 0 (success) or 2 (critical error). As for 1,

### Run for specified duration without shell

Most of the time, your ælhometta will run without any interference from you, hours after hours, maybe months after months, until you interrupt its silent course by pressing any key.

![routine scast gif](https://raw.githubusercontent.com/aelhometta/visuals/main/aelhometta_routine.gif)

(Nowhere as fancy as the demo in synopsis.)

For the sake of resiliency, however, it is recommended to backup the state regularly.

These approaches combine when you run the application with single argument instead of no arguments, that argument being the requested duration of running in seconds:

```shell
$ ./aelhometta 43200
```

There is no shell in this mode. As soon as the duration ends (12 hours in this example) *or* a key is pressed, the application exits and saves the state. In the latter case, exit status is set to 1.

To run Ælhometta indefinitely with backup once per hour, place such call into a loop:

```shell
#!/bin/sh

while true; do
    ./aelhometta 3600
    if [ $? = 1 ]; then
        echo "Halt due to keypress"
        break
    fi
done
```

Be aware that this loop will continue in case of the application's critical error (exit status 2).

To run Ælhometta for one day each week, place the `/path/aelhometta 86400` call into `/etc/cron.weekly/`.

Whatever the scenario of this kind is, it may help to imagine your character in the scenario being — absent, far away, gone, you name it, except for brief appearance in the beginning. Which is how the things are going to be anyway...

## Basic elements of ælhometta

...Perhaps the better way to acquaint yourself with it is to simply read through `src/aelhometta.rs`. The namesake structure verbatim from there:

```rust
pub struct Ælhometta {
    // Serialisable part
    max_num_chains_binlog: u8,

    new_node_uid: Uid,
    nodes: HashMap<Uid, Node>,
    nodes_historing: Vec<Optuid>,
    i_nodes_historing: usize,

    new_controller_uid: Uid,
    controllers: HashMap<Uid, Controller>,
    controllers_historing: Vec<Optuid>,
    i_controllers_historing: usize,

    introspection: bool,

    ether_optuids: Vec<Optuid>,
    ether_integers: Vec<Integer>,

    age: u128,
    commands_count: HashMap<Command, u128>,

    glitch_background_prob: f64,
    glitch_background_count: u128,
    glitch_replicate_prob: f64,
    glitch_replicate_count: u128,
    glitch_construct_prob: f64,
    glitch_construct_count: u128,

    // Peer-related
    share_size: usize,
    share_interval: i64,

    ut_last_share: i64,

    secretkey: String,
    port: u16,
    torproxy_port: u16,
    torproxy_host: String,

    exposed: bool,

    other_peers: Vec<OtherPeer>,

    whitelist: HashSet<String>,

    // IO-related
    output_mappings: Vec<IntegersFileMapping>,
    input_mappings: Vec<IntegersFileMapping>,

    // Non-serialisable part
    max_num_chains: usize,
    max_num_chains_binmask: usize,

    rng: ThreadRng,

    efunguz: Option<Efunguz>,
}
```

...Ah, of course, some omnipresent pseudonyms:

```rust
pub type Uid = u32;
pub type Optuid = Option<Uid>;

pub type Integer = i64;
```

<details>
<summary><b>Nodes</b></summary>

Single-linked units of information. See [A node](#a-node).

</details>

<details>
<summary><b>Controllers</b></summary>

Automata that move along chains of nodes and act accordingly to the content of these nodes. See [A controller](#a-controller).

</details>

<details>
<summary><b>2 Ethers</b></summary>

Global arrays of opaque node addresses and integer values, accessible to all controllers.

</details>

<details>
<summary><b>Peer configuration</b></summary>

Specifies network identity of ælhometta and its interaction with other ælhomettas across the network (Tor, to be more precise). See [Networking](#networking).

</details>

<details>
<summary><b>I/O configuration</b></summary>

Maps continuous ranges of integers ether from (input) and to (output) files controlled by other applications. Those applications, in turn, connect the files with sensors and actuators in outside world. See [Input/Output](#inputoutput).

</details>

<details>
<summary><b>Miscellaneous</b></summary>

`introspection`, when `true`, unlocks `GetExecFromOptuid` and `SetOptuidFromExec` commands (see [Command](#command) soon below). They deviate from descriptive/executive separation principle, and therefore this switch is `false` by default.

`age` increments each tick. `commands_count` keeps track of how many times each available command has been executed.

---

The state of ælhometta is saved to `aelhometta.bin`, see `src/aelhometta/serbin.rs`. This serialisation, though binary and with tricks such as [LEB128](https://en.wikipedia.org/wiki/LEB128), is not minimal in size; classical ZIP, for instance, nearly halves it.

</details>

## A node

```rust
pub struct Node {
    b_content: u8,
    b_next: Uid,
    b_altnext: Uid
}
```

A node has *content*, which describes what the controller should do, and 2 pointers: *(main) next node* and *alternative next node*, which describe to what node the controller moves after this one.
The value of such pointer (it may be empty) is also called *optuid* (optional unique identifier).

The content is represented as standard byte, see `impl ToBits<u8> for Content` and `impl OtBits<u8> for Content` in `src/aelhometta.rs`.

There are 4 types of content:

### Space

[NOP](https://en.wikipedia.org/wiki/NOP_(code)), placeholder, does nothing. But it can be replaced with something.

### Branch

This type of node is the only one providing non-linearity of execution path, if *introspection* is off.

If the `success` flag is `true`, execution pointer moves to the main next node.

If the `success` flag is `false`, execution pointer moves to the alternative next node.

### Command

Specifies an action that controller should perform. May change controller's state — registers, flags, arrays of pointers, integers, and so on (see [A controller](#a-controller)); may also change the state of the entire ælhometta.

If a command fails (e.g. division by 0, overflow, index out of bounds), the `success` flag will be set to `false`. However, some "junk" may be present where result should have been, and what is "failure" and what is not depends. `Test...` commands affect `success` flag too.

"Uncrashability" principle (single chemical reaction cannot crash the universe) permeates what commands do and is familiar to everyone in the trade.

Again, `src/aelhometta/tick.rs` provides more complete picture. The following list heavily relies on self-explanatory property of... words.

<details>
<summary><b>Available commands</b></summary>

* Nullary operators with integer result placed in the integer register:

    * `RandomContent` ("valid" integer representing one of `Content` variants)
    * `RandomInteger` (all 64 bits are random)
    * `ZeroInteger`

* Unary operators on integer register replacing its value with the result:

    * `Abs`
    * `BitNot`
    * `Decrement`
    * `Increment`
    * `Negate`
    * `ShiftUp`
    * `ShiftDown`
    * `Sign`
    * `Square`

* Binary operators on integer register as the 1st operand and selected integer as the 2nd one, the result goes to integer register:

    * `Add`
    * `BitAnd`
    * `BitOr`
    * `BitXor`
    * `Divide`
    * `Multiply`
    * `Remainder`
    * `Subtract`

* Convert integer register to index of selected element in certain array...

    * `IntegerToDataOptuidIndex`
    * `IntegerToIntegerIndex`
    * `IntegerToIntegerChannel`
    * `IntegerToOptuidChannel`
    * `IntegerToOptuidIndex`
    * `IntegerToPeer`

* ...and back:

    * `DataOptuidIndexToInteger`
    * `IntegerChannelToInteger`
    * `IntegerIndexToInteger`
    * `OptuidChannelToInteger`
    * `OptuidIndexToInteger`
    * `PeerToInteger`

* Convert integer register to success flag and vice versa (usual int ↔ bool semantics):

    * `IntegerToSuccess`
    * `SuccessToInteger`

* Operations with node pointed to by `data_optuids[i_data_optuid]`:

    * `Insert`
    * `Read`
    * `Remove`
    * `Skip`
    * `Write`

* Tests, affecting `success` flag:

    * `TestDataOptuid` (true if `data_optuids[i_data_optuid]` points to existing node)
    * `TestIntegerNegative`
    * `TestIntegerNonZero`
    * `TestIntegerPositive`

* Creation of a new chain and, if it is active, of a controller attached to it:

    * `Construct`
    * `NewChainAddInteger`
    * `NewChainAddIntegerChannel`
    * `NewChainAddOptuid`
    * `NewChainAddOptuidChannel`
    * `NewChainDetach`
    * `NewChainInitActive`
    * `NewChainInitPassive`
    * `Replicate`

`Construct` and `Replicate` work with `data_optuids[i_data_optuid]` of a controller, consequtively reading the node where it points and advancing it to the next node. In a sense, they are "shortcuts", since they cut some corners of Ælhometta's artificial chemistry.

`NewChainAdd...` actually add respective element to the *controller* attached to the active chain being created.

A new chain is not empty, it has `Space` node at the beginning.

* Move to next/previous element of corresponding array:

    * `NextDataOptuid`
    * `NextInteger`
    * `NextIntegerChannel`
    * `NextOptuid`
    * `NextOptuidChannel`
    * `NextPeer`
    * `PreviousDataOptuid`
    * `PreviousInteger`
    * `PreviousIntegerChannel`
    * `PreviousOptuid`
    * `PreviousOptuidChannel`
    * `PreviousPeer`

* Read/write "optuids ether" — global array of (some) optuids accessible to all controllers of ælhometta, with destination/source respectively being currently selected optuid of a controller, and the index of ether's element being provided by `optuid_channels[i_optuid_channel]`:

    * `ReceiveOptuid`
    * `TransmitOptuid`

* Read/write "integers ether". Destination/source is integer register, ether's element is given by `integer_channels[i_integer_channel]`, and the ether is that of other peer when `i_peer` is not 0. In the latter case, the ether is read-only (only `Receive...` works) since it has been obtained from other peer via publish-subcribe pattern:

    * `ReceiveInteger`
    * `TransmitInteger`

* Exchange data between integer register and integers array, and between advancing and non-advancing optuids arrays:

    * `GetIntegerFromIntegers`
    * `SetDataOptuidFromOptuid`
    * `SetIntegersFromInteger`
    * `SetOptuidFromDataOptuid`

* Restart controller:

    * `Restart`

* Copy selected optuid to exec optuid (this command is unique in "forcing" the optuid of next execution node regardless of current node's main and alternative pointers) and vice versa. *These two work only if `introspection` switch is set to `true` (default is `false`)*, otherwise they are equivalent to `Space`:  

    * `GetExecFromOptuid`
    * `SetOptuidFromExec`

Note that it is impossible to convert Optuid to Integer and vice versa, in accordance with "opaque addressing" principle.

Also, note redundancy. For example, `NextIntegerChannel` does almost what `IntegerChannelToInteger`, `Increment`, `IntegerToIntegerChannel` do.

</details>

### Construction

Comes into play only when a controller constructs an executive (active) chain, — when `Construct` command is executed, which works with the stack of nodes' uids.

<details>
<summary><b>Available construction instructions</b></summary>

* `AltNext` turns on "alternative next" mode until `NextToStored` instruction, which will then set the alternative next pointer of currently added node instead of the main one and revert to "main mode"
* `Discard` removes topmost uid from the stack
* `NextToStored` sets main or alternative next node pointer of the currently added node to the topmost uid of the stack
* `Restore` changes "construction pointer" from currently added node to the topmost uid of the stack
* `Store` pushes the uid of the currently added node to the stack
* `Swap` swaps 2 topmost uids on the stack
* `Terminus` interrupts the `Construct` (but not `Replicate`) command at currently added node

</details>

---

Still we lack any quantification or algebraisation of the intricate ways in which the choice of encoding by natural numbers affects the evolutionary perspectives of such system... For example, variants of `enum Command` are numbered in alphabetical order: `ReceiveOptuid` is `52`, `Remainder` is `53`, similarly, `AltNext` is `0` and `Terminus` is `6`; why not contrariwise? This is so arbitrary, so torn away from underlying levels, as if chemistry were decoupled from physics, that evolution may be too weak to fill the gap... with what? Even that is *innominabilis*.

**Remark.** There were an older version of Ælhometta without automatic conversion between `Content` and `Integer`, with 2 respective registers instead of 1 `Integer` now. That approach implied too much opacity of the numerical level as seen from the instruction level, and was abandoned. We leave its resurgence as "an exercise to the reader" (see also [Panmutations](#panmutations)).

## A controller

Executive entity. CPU is another analogy.

Again, verbatim from the source:

```rust
pub struct Controller {
    chain_start_optuid: Optuid,

    exec_optuid: Optuid,

    data_optuids: Vec<Optuid>,
    i_data_optuid: usize,

    new_chain_optuid: Optuid,
    new_controller: Option<Box<Self>>,

    registers: Registers,
    flags: Flags,

    optuids: Vec<Optuid>,
    i_optuid: usize,

    integers: Vec<Integer>,
    i_integer: usize,

    optuid_channels: Vec<usize>,
    i_optuid_channel: usize,

    i_peer: usize,

    integer_channels: Vec<usize>,
    i_integer_channel: usize,

    generation: u128,
    ticks: u128
}
```

For now, plural in `Registers` and `Flags` is redundant, because

```rust
pub struct Registers {
    integer: Integer
}

pub struct Flags {
    success: bool
}
```

`exec_optuid` is basically the instruction pointer, `chain_start_optuid` keeps its initial value for the sake of `Restart` command. Controller "dies" as soon as `exec_optuid` becomes `None` or points to non-existing node.

`new_chain_optuid` and `new_controller` are used for replication of a passive/descriptive chain and construction of an active/executive chain (one with a controller attached to it).

`data_optuids[i_data_optuid]` advances automatically to the next node at read/write operations realised by `Read`, `Write`, and `Insert` commands, which use this optuid. `Construct` and `Replicate` commands advance it too, "until the end".

`i_data_optuid`, `i_optuid`, `i_integer`, `i_optuid_channel`, and `i_integer_channel` are indices of currently selected elements of respective arrays: `data_optuids[]` etc.

`i_peer`, when 0, means "this one", otherwise it means other peers, enumerated from 1. It affects interpretation of `integer_channels`, e.g. `Transmit` command works only for this peer.

`generation` is set once at the creation of a controller to `generation` of the constructing controller + 1. `ticks` is 0 at controller's creation and increments at each its... tick.

## Ancestors...

The simpler one, **ancestor B**, consists of 

* *Constructor's scheme chain*

```
Construction(Store),

// Replicate scheme
Command(SetDataOptuid),
Command(NextOptuid),
Command(NewChainInitPassive),
Command(PreviousOptuid),
Command(Skip),
Command(Replicate),
Command(NewChainDetach),

// Build constructor from scheme
Command(SetDataOptuid),
Command(NextOptuid),
Command(NextOptuid),
Command(NewChainInitActive),
Command(Skip),
Command(Construct),
Command(PreviousOptuid),
Command(NewChainAddOptuid),
Command(NewChainDetach),

Construction(NextToStored),
Construction(Discard)
```

* *Constructor's executive chain*, which is almost the same as the scheme, except for the absence of Construction nodes and, <u>accordingly to instructions from these nodes</u>, the last node pointing to the implicit 1st `Space` node: this chain is a loop. (Well, not exactly: from 1st generation onward. In 0th generation, which is the ancestor itself, the loop is open.)

* *Constructor's controller* attached to constructor's executive chain.

Note the classical double interpretation of data: first is linear verbatim copying, second is non-linear construction that expands the meaning of special (`Construction:...`) units. Will an evolution keep the separation line between them clear?

*Spacity* parameter that you provide at introducing this ancestor — as in `@ anc b 5` — is the number of `Space`-s (5 in this case) inserted before every non-`Construction` node. They are *nothing* for mutations to replace with *something* without the need to destroy the original sequence of actions.

This ancestor utilises only a small fraction of available `Content`-s. There is also slightly more complicated, with larger assortment of `Content`-s, **ancestor A**: in addition to constructor, it has *jumbler* that scans the same scheme constructor uses and randomly changes it (by replacements, insertions, deletions). In other words, jumbler interiorises mutations. However, in Ælhometta, the standard way to introduce [Mutations](#mutations) is "external" and more global at that.

See also `src/aelhometta/ancestors.rs`.

## ...and Descendants

The following chains have been extracted at random from an ælhometta with mutations and tiny input mapping from microphone, during 3 days of running. Maximum allowed number of nodes is 2<sup>24</sup>=16777216, same for controllers (although there never have been more than 3×10<sup>5</sup> of them).

### After ~3×10<sup>8</sup> ticks

<details>
<summary><b>Evolved chain, example 1</b></summary>

```
Space
Command:NextDataOptuid
Space
Space
Command:ZeroInteger
Space
Command:NewChainAddOptuidChannel
Space
Command:Abs
Command:Remove
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:Write
Command:PreviousIntegerChannel
Command:NewChainAddOptuid
Command:NextPeer
Command:Construct
Command:NewChainDetach
Command:GetExecFromOptuid
Command:IntegerIndexToInteger
Command:SetIntegersFromInteger
Command:ZeroInteger
Space
Command:NextIntegerChannel
Command:OptuidChannelToInteger
Command:Construct
Space
Command:Read
Command:Negate
×
```
</details>

<details>
<summary><b>Evolved controller chain, example 1</b></summary>

```
Space
Space
Space
Space
Space
Command:BitNot
Space
Command:IntegerToOptuidChannel
Command:OptuidChannelToInteger
Command:SetOptuidFromExec
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:Subtract
Command:GetIntegerFromIntegers
Command:NewChainAddOptuid
Command:NextPeer
Command:Construct
Command:NewChainDetach
Command:GetExecFromOptuid
Command:GetExecFromOptuid
Command:BitAnd
Command:ZeroInteger
Space
Command:GetExecFromOptuid
Command:Write
Command:PreviousDataOptuid
Space
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:Write
Command:IntegerToIntegerIndex
Command:NewChainAddOptuid
Command:ShiftUp
Command:Construct
Command:NewChainDetach
Command:GetExecFromOptuid
Space
Command:SetIntegersFromInteger
Command:ZeroInteger
Space
Command:GetExecFromOptuid
Command:Increment
Command:PreviousOptuidChannel
Space
Command:GetIntegerFromIntegers
Command:NextIntegerChannel
Command:TestIntegerNegative
Command:NewChainInitActive
Command:TestIntegerNonZero
Command:BitOr
Command:NewChainAddOptuid
Command:GetExecFromOptuid
Command:NewChainAddOptuid
Command:SetIntegersFromInteger
Command:PreviousDataOptuid
Command:Restart
Command:RandomInteger
Command:NewChainAddOptuidChannel
Command:NewChainAddInteger
Command:BitNot
Command:IntegerToDataOptuidIndex
Command:RandomContent
Space
Command:IntegerToIntegerIndex
Command:Remainder
Command:Remove
Space
Command:TransmitOptuid
Command:OptuidIndexToInteger
Command:OptuidChannelToInteger
Command:PreviousOptuid
Command:Skip
Command:Replicate
Command:Divide
Space
×
```
</details>

### After ~4×10<sup>8</sup> ticks

<details>
<summary><b>Evolved chain, example 2</b></summary>

```
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:BitNot
Command:Skip
Command:NewChainAddOptuid
Command:NextPeer
Command:Construct
Command:NewChainDetach
Command:Write
Command:BitAnd
Command:Add
Space
Command:Write
Command:Read
Space
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:Write
Command:Remainder
Command:NewChainAddOptuid
Command:Divide
Command:Construct
Command:NewChainDetach
Command:GetExecFromOptuid
Space
Command:Read
Command:NewChainDetach
Command:Read
Command:ZeroInteger
Command:Increment
Command:PreviousOptuidChannel
Space
Branch
Command:RandomContent
Command:TestIntegerNegative
Command:Construct
Space
Command:BitOr
Command:TestIntegerNegative
Command:PreviousInteger
Command:Remove
Command:OptuidChannelToInteger
Command:PreviousDataOptuid
Command:NextOptuidChannel
Command:SetIntegersFromInteger
Command:NewChainAddOptuidChannel
Command:NewChainAddOptuidChannel
Command:NewChainInitPassive
Command:IntegerToDataOptuidIndex
Command:Write
Space
Command:IntegerToIntegerIndex
Command:Restart
Command:Remove
Space
Command:Decrement
Command:TransmitOptuid
Command:OptuidIndexToInteger
Command:TestIntegerPositive
Command:NextDataOptuid
Command:Abs
Command:Divide
Space
×
```
</details>

<details>
<summary><b>Evolved controller chain, example 2</b></summary>

```
Space
Command:OptuidIndexToInteger
Space
Command:NextInteger
Space
Space
Command:NewChainAddOptuid
Command:PreviousDataOptuid
Command:OptuidChannelToInteger
Command:IntegerToSuccess
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:Write
Command:Replicate
Command:NewChainAddOptuid
Command:Construct
Command:NewChainDetach
Command:TestDataOptuid
Command:SetOptuidFromExec
Command:TestIntegerNegative
Command:IntegerToPeer
Command:NewChainDetach
Command:GetExecFromOptuid
Command:Increment
Command:PreviousDataOptuid
Command:SetDataOptuidFromOptuid
Command:TestIntegerNonZero
Command:Write
Command:GetIntegerFromIntegers
Command:NewChainAddOptuid
Command:Divide
Command:Construct
Command:NewChainDetach
Command:GetExecFromOptuid
Command:IntegerToDataOptuidIndex
Command:RandomContent
Space
Space
Command:SetOptuidFromExec
Command:Construct
Command:ShiftDown
Space
Command:NextIntegerChannel
Command:TestIntegerNonZero
Command:NewChainInitActive
Space
Command:BitOr
Command:Construct
Command:Decrement
Command:Remove
Command:SetIntegersFromInteger
×
```
</details>

### After ~5×10<sup>8</sup> ticks

<details>
<summary><b>Evolved chain, example 3</b></summary>

```
Space
Space
Space
Space
Space
Space
Space
Space
Space
Space
Space
Space
Space
Space
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:IntegerToIntegerIndex
Command:TestIntegerPositive
Command:NewChainAddOptuid
Command:Construct
Command:NewChainDetach
Command:PreviousInteger
Command:PreviousOptuid
Command:BitAnd
Command:TestDataOptuid
Command:GetExecFromOptuid
Command:Write
Command:Increment
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:Subtract
Command:ReceiveInteger
Command:NewChainAddOptuid
Command:Divide
Command:PreviousInteger
Command:NewChainDetach
Space
Command:PreviousIntegerChannel
Command:ZeroInteger
Space
Command:GetExecFromOptuid
Command:NextPeer
Command:IntegerToOptuidIndex
Space
Command:NextOptuid
Command:Subtract
Command:Add
Command:NewChainInitActive
Command:Add
Command:BitOr
Command:PeerToInteger
Command:GetExecFromOptuid
Command:Remove
Command:NewChainInitActive
Command:NewChainAddOptuidChannel
Command:PreviousInteger
Command:BitXor
Command:SetIntegersFromInteger
Command:NewChainAddInteger
Command:DataOptuidIndexToInteger
Command:Subtract
Command:PreviousIntegerChannel
Space
Command:Write
Command:Remainder
Command:OptuidIndexToInteger
Space
×
```
</details>

<details>
<summary><b>Evolved controller chain, example 3</b></summary>

```
Space
Space
Space
Space
Command:Insert
Command:Write
Command:IntegerIndexToInteger
Command:OptuidChannelToInteger
Command:PeerToInteger
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:Write
Command:NewChainAddOptuid
Command:Multiply
Command:Construct
Command:NewChainDetach
Command:GetExecFromOptuid
Command:SetDataOptuidFromOptuid
Command:BitAnd
Command:ReceiveInteger
Command:Insert
Command:GetExecFromOptuid
Command:NewChainAddOptuid
Command:PreviousDataOptuid
Command:SetDataOptuidFromOptuid
Command:NewChainInitActive
Command:NextOptuid
Command:SetOptuidFromDataOptuid
Command:NewChainAddOptuid
Command:IntegerToPeer
Command:IntegerIndexToInteger
Command:BitOr
Command:Subtract
Command:PreviousPeer
Command:ZeroInteger
Command:PreviousOptuidChannel
Command:BitXor
Command:IntegerToIntegerChannel
Command:Restart
Space
Command:NextDataOptuid
Command:Multiply
Command:Increment
Command:SetIntegersFromInteger
Command:BitOr
Command:PeerToInteger
Command:GetExecFromOptuid
Command:Remove
Command:Write
Command:IntegerToDataOptuidIndex
Command:NewChainInitActive
Command:PreviousInteger
Command:BitXor
×
```
</details>

**Remark.** It is quite possible for the first chain of a pair to belong to a controller as well, rather than to a scheme. To be sure, check for `Construction`-s: they can appear in a chain built by `Replicate`, and they cannot appear in a chain built by `Construct`.

---

Enjoy the mess and repetitiveness of evolution... Well, what evolutionary conclusions can we draw of this sample? At first, there seems to be a lot of junk, but is it really junk? can it be removed without significant changes in "phenotype"? or does it interact with parts not shown here in nontrivial ways? In particular, numerical operations (`Add`, `Divide`, `ShiftUp`) are interspersed everywhere.

`Construct` and `Replicate` commands, with `NewChainInit...` and `NewChainDetach`, indicate the ability to procreate. Without states of controllers and chains their `Optuid`-s and `DataOptuid`-s point to, we cannot say how "vital" their children will be.

Loops in original ancestors, and non-linearities of execution path in general, have mostly been lost along the evolution road, except for occasional `Branch` (chain 2).

`Construction` instructions remain very rare.

There is some access to global arrays — `TransmitOptuid`, `ReceiveInteger` commands — but, again, without the rest it is hard to say whether this access is meaningful, at least are there `ReceiveOptuid`, `TransmitInteger` counterparts somewhere else.

How many eons away is this from the level of *E.coli*<sup>[[GLA1]](#refGLA1)</sup>?..

## Mutations

Here we call them *glitches*.

* *Background* glitch occurs at each tick with specified probability and randomly changes the content of random node of entire ælhometta

* *Replication* glitch occurs at replication for each replicated node with specified probability and changes its content randomly as well

* *Construction* glitch occurs at construction for each read node with specified probability, changing its content randomly

To be more precise, "randomly" means equiprobably.

By default, all three probabilities are 0. We've shown how to adjust them in [Quickstart](#quickstart). To see their values and the counts of corresponding glitches that have occured is even simpler:

```
@ glitch
```

<details>
<summary><b>Example of content statistics <i>without</i> glitches (or jumblers)</b></summary>

```
Command:Remainder                                  0      0.000 %
Command:Remove                                     0      0.000 %
Command:Replicate                              38860      1.012 %
Command:Restart                                    0      0.000 %
Command:SetDataOptuid                          77720      2.023 %
Command:SetInteger                                 0      0.000 %
Command:SetOptuid                                  0      0.000 %
Command:ShiftUp                                    0      0.000 %
Command:ShiftDown                                  0      0.000 %
Command:Sign                                       0      0.000 %
Command:Skip                                   77720      2.023 %
Command:Square                                     0      0.000 %
Command:Subtract                                   0      0.000 %
Command:SuccessToInteger                           0      0.000 %
Command:TestDataOptuid                             0      0.000 %
Command:TestIntegerNegative                        0      0.000 %
Command:TestIntegerNonZero                         0      0.000 %
Command:TestIntegerPositive                        0      0.000 %
Command:TransmitInteger                            0      0.000 %
Command:TransmitOptuid                             0      0.000 %
Command:Write                                      0      0.000 %
Command:ZeroInteger                                0      0.000 %
Construction:AltNext                               0      0.000 %
Construction:Discard                           22393      0.583 %
Construction:NextToStored                      22393      0.583 %
```
</details>

<details>
<summary><b>Example of content statistics <i>with</i> glitches</b></summary>

```
Command:Remainder                                937      0.022 %
Command:Remove                                   274      0.007 %
Command:Replicate                              42305      1.009 %
Command:Restart                                  268      0.006 %
Command:SetDataOptuid                          84855      2.023 %
Command:SetInteger                              1065      0.025 %
Command:SetOptuid                                516      0.012 %
Command:ShiftUp                                  528      0.013 %
Command:ShiftDown                               1311      0.031 %
Command:Sign                                   4261.017 %
Command:Skip                                   43337      1.033 %
Command:Square                                   270      0.006 %
Command:Subtract                                1010      0.024 %
Command:SuccessToInteger                         558      0.013 %
Command:TestDataOptuid                           782      0.019 %
Command:TestIntegerNegative                      523      0.012 %
Command:TestIntegerNonZero                      1332      0.032 %
Command:TestIntegerPositive                      665      0.016 %
Command:TransmitInteger                         1202      0.029 %
Command:TransmitOptuid                          1798      0.043 %
Command:Write                                   1135      0.027 %
Command:ZeroInteger                             1315      0.031 %
Construction:AltNext                             258      0.006 %
Construction:Discard                             281      0.007 %
Construction:NextToStored                        225      0.005 %
```
</details>

### Panmutations

Mutations are limited in that they cannot change the set of available commands, how commands work, general structure of ælhometta... all these things are "above" (<u>πανω</u> απο) them. For now, the only potential source of such <u>pan</u>mutations is... *you*, as a programmer, irritated by our design choices and anxious to rewrite some especially crappy parts of Ælhometta. *Welcome!* at least as long as **[Networking](#networking) and [I/O](#inputoutput) protocols remain compatible**, because then your ÆlhomettaPlus and all other versions panmutated differently by fellow rewriters will consolidate into an abiosphere.

In time, perhaps, ælhomettas will obtain means to panmutate themselves, e.g. rewriting and recompiling their source through specialised I/O... and, which is where the present overtakes, through tools such as [Copilot](https://en.wikipedia.org/wiki/GitHub_Copilot).

## Networking

Each Ælhometta instance is a potential peer, identified from the outside by its *public key*, *onion address*, and *port*. To confirm the "right" to use the public key, the corresponding *secret key* must be specified.

The peers exchange data following the [publish-subscribe pattern](https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern): each ælhometta shares some continuous subset of its integer channels, from the beginning of their array (because channels with small indices seem to be used more often as evolution unfolds), and every other ælhometta that has subscribed to it receives this subset (if there is no whitelist filter at publisher side). We anticipate complaints against this pattern being too "passive": one ælhometta cannot say anything to another ælhometta until the latter one initiates listening.

Inherently, there is no central server, rather every peer is a server for peers "interested" in the data it provides. Neither this is torrent-like, because a tracker is absent: you must know exactly the (public key, onion, port) identity of a peer to subscribe to it.

The underlying messaging library is [ZeroMQ](https://zeromq.org/), thus both [Curve](https://rfc.zeromq.org/spec/26/) keys, public and secret ones, are 40-character [Z85](https://rfc.zeromq.org/spec/32/) strings. Obtain them via a call to `zmq_curve_keypair()` from original [libzmq](https://github.com/zeromq/libzmq) or via its wrapper from numerous language bindings. [Quickstart](#quickstart) shows how to do it in Python.

Network identities and data flow are provided by [onion services](https://community.torproject.org/onion-services/overview/) (v3) of [Tor](https://www.torproject.org/).

Note that public key of ZeroMQ is not related to public key of onion service. There is double encryption/authentication here, which is probably redundant...

---

After `@ peer expose` your peer starts publishing data every `interval` microseconds, first `size` integers from your ælhometta's integers ether (0th, 1st, (`size` - 1)th).

Subscription to other peer can be stopped at any time:

```
@ peer disconnect TheirPublicKeyTheirPublicKeyTheirPublicK
```

At that, indices of all subsequent peers decrement. If ælhometta has tuned to such indices (`i_peer` of its controllers), the tuning will probably be lost. It seems safer to add peers than to remove them.

You can stop the entire network activity, both transmitting and receiving, whenever you want:

```
@ peer repose
```

Last data obtained from each other peer is kept, though, as long as you do not initiate a disconnection.

There is no requirement to transmit *and* receive, but the secret key has to be specified even if you need only to receive. As long as `interval` equals 0, there will be no transmission. On the other hand, if `interval` > 0 and `size` = 0, your peer will transmit empty shares (usable as keepalives).

You can restrict peers that are able to subscribe to your peer by adding them to *whitelist* (if it is empty, all others are allowed):

```
@ peer whitelist add TheirPublicKeyTheirPublicKeyTheirPublicK
```

If circumstances change, any such key (and corresponding peer) can be deleted from whitelist via `peer whitelist del ...`. Or restrictions can be removed alltogether via `peer whitelist clear`.

Without whitelist, anyone in the world who knows the public key, the onion address, and the port, is able to subscribe; there is no way to predict how many subscribers your ælhometta will have at certain time in the future, so the Internet traffic may vary.

<details>
<summary><b>Known peers out there</b></summary>

Public key | Onion | Port | Description | Share size
-----------|-------|------|-------------|-----------
`USBD7[O^8L[}saCh+6U#}6wie4oAedZ4#W4!b%LI` | `t3kauc3flp2bpv3mv7vtnedfu6zrho3undncccj35akpuyjqqydwvfyd` | `60847` | Maintained by us for testing. Online rather than offline | 1000–10000
`&i!VkHl[]m.c!^j0D%i)&4#[u5b(a=QCdZ9C0$p{` | `yhel64h6cjab75tcpncnla2rdhqmxut2vtitywhbu7bpjh4hfhp6hnid` | `60847` | Maintained by us for testing. Offline rather than online | 1000–10000

</details>

---

Please be careful: you interact with *any* other peer at your own risk. One of security concerns is **size limit** — you probably do not want to spend traffic, depleting a tariff of your provider, to receive several gigabytes of someone's generously shared... zeros (`00 00 ... 00`) and then crash with "Out of memory!"

Another concern is how the data received from untrusted sources affects your ælhometta, what ideas it can develop... in whose interests it will operate...

### Transferring network identity

That is, at moving your ælhometta to another computer.

Besides `aelhometta.bin`, you need to keep the content of Tor hidden service dir, which itself is inside `/var/lib/tor/` on Linux. 3 essential files there are `hostname`, `hs_ed25519_public_key`, `hs_ed25519_secret_key`. This structure must be recreated on the next computer, along with `/etc/tor/torrc` or at least its `HiddenServicePort` and `HiddenServiceDir` settings.

Make sure that the ælhometta has become online on the new place (others receive its shares), *then* remove it from the old one or do not expose it to the network from there, so that Tor will not be confused by two onions with the same identity.

## Input/Output

Ranges of integer channels can be mapped *from* (input) or *to* (output) files with verbatim — little endian, 8-byte — representations of the integers. The programs working with such files can be completely independent of Ælhometta, except for some synchronisation of "tempo" (`interval`, in microseconds).

Output files are truncated and overwritten at each update.

Size of an input file must be no less than 8 times the length of the range of integer channels to which it is mapped, otherwise updates do not happen.

```rust
pub struct IntegersFileMapping {
    start: usize,
    length: usize,
    interval: i64,
    filepath: String,
    ut_last_update: i64,
}
```

All output mappings are synchronised with corresponding files before all input mappings — with theirs<sup>[[BUZ1]](#refBUZ1)</sup>.

We have considered the usage of `iomap` command in [Quickstart](#quickstart). There, external programs to analyse (input, "hearer") and synthesise (output, "buzzer") sound were black boxes: from ælhometta's point of view, they only have to write and read, respectively, files whose sizes are 8 times the lengths of mapped ranges. Let us shed light into blackness... one of many possible ways to do it, e.g. in Python:

<details>
<summary><b>aelhom_hearer.py</b></summary>

```python
import numpy as np
import sounddevice as sd

NUM_BANDS = 14
MIN_FREQUENCY = 100
MAX_FREQUENCY = 6000
DESTINATION_FILEPATH = "./hear.i64"
SAMPLE_RATE = 32768
UPDATE_RATE = 2.0

BASIC_FREQUENCIES = [int(MIN_FREQUENCY * np.power(2.0, i * np.log2(MAX_FREQUENCY / MIN_FREQUENCY) / (NUM_BANDS - 1))) for i in range(NUM_BANDS)] # in Hz
NUM_REC_SAMPLES = int(SAMPLE_RATE / UPDATE_RATE)

print("Basic frequencies (Hz):", BASIC_FREQUENCIES)
print("Press Ctrl+C to exit...")

samples = np.zeros(SAMPLE_RATE)

updates = 0

try:
	while True:
		recording = sd.rec(NUM_REC_SAMPLES, samplerate=SAMPLE_RATE, channels=1, dtype='int16', blocking=True).flatten()

		if NUM_REC_SAMPLES < SAMPLE_RATE:
			samples = np.concatenate((samples[NUM_REC_SAMPLES:], recording))
		else:
			samples = recording[(NUM_REC_SAMPLES - SAMPLE_RATE):]

		spectrum = np.absolute(np.fft.rfft(samples)[1:])
		begin = 0
		bandspectrum = np.zeros(NUM_BANDS)
		for i in range(NUM_BANDS):
			end = BASIC_FREQUENCIES[i]
			# bandspectrum[i] = np.sum(spectrum[begin:end])
			# bandspectrum[i] = np.average(spectrum[begin:end])
			bandspectrum[i] = np.max(spectrum[begin:max(begin + 1, end)])
			begin = end

		bandspectrum /= max(np.max(bandspectrum), 1e-8)

		updates += 1
		status_str = f"[{updates}] Bandspectrum: "

		bs = bytes()
		for i in range(NUM_BANDS):
			i64 = int(0xFF * bandspectrum[i]) # only lowest byte of 8
			bs += i64.to_bytes(8, byteorder="little")
			status_str += f"{i64:02X} "
		
		with open(DESTINATION_FILEPATH, "wb") as f:
			f.write(bs)

		print(status_str, end="\r", flush=True)

except KeyboardInterrupt:
	print("Done.")
```
</details>

<details>
<summary><b>aelhom_buzzer.py</b></summary>

```python
import numpy as np
import pygame as pg

import time

NUM_BANDS = 12
MIN_FREQUENCY = 150
MAX_FREQUENCY = 5000
SOURCE_FILEPATH = "./buzz.i64"
SAMPLE_RATE = 4
UPDATE_RATE = 1.0
IDLE_RATE = 100.0

BASIC_FREQUENCIES = [int(MIN_FREQUENCY * np.power(2.0, i * np.log2(MAX_FREQUENCY / MIN_FREQUENCY) / (NUM_BANDS - 1))) for i in range(NUM_BANDS)] # in Hz

print("Basic frequencies (Hz):", BASIC_FREQUENCIES)

pg.mixer.init(frequency=SAMPLE_RATE, channels=1)
pg.mixer.set_num_channels(NUM_BANDS)

pitches = [pg.sndarray.make_sound(np.array(32767.0 * np.sin(np.linspace(0.0, 2.0 * np.pi * f, SAMPLE_RATE) + np.random.random() * 2.0 * np.pi), dtype='int16')) for f in BASIC_FREQUENCIES] # clear tones

volumes = [0.0 for i in range(NUM_BANDS)]

for p in pitches:
	p.set_volume(0.0)
	p.play(-1)

t_last_update = - 1.0 / UPDATE_RATE - 1.0 # ensure immediate update
print("Press Ctrl+C to exit...")

updates = 0

try:
	while True:
		t = time.time()
		if t - t_last_update >= 1.0 / UPDATE_RATE:
			updates += 1

			status_str = f"[{updates}] Volumes: "

			try:
				with open(SOURCE_FILEPATH, "rb") as f:
					fcontent = f.read(NUM_BANDS << 3) # 64-bit integers
					if len(fcontent) == NUM_BANDS << 3:
						for i in range(NUM_BANDS):
							i64 = int.from_bytes(fcontent[(i << 3):((i + 1) << 3)], byteorder="little", signed=True)
							vol = abs(i64) & 0xFF # only lowest byte matters
							volumes[i] = vol / 0xFF
							status_str += f"{vol:02X} "

				for i in range(NUM_BANDS):
					pitches[i].set_volume(volumes[i])

			except FileNotFoundError:
				status_str += "source file not found"

			t_last_update = t

			print(status_str, end="\r", flush=True)
		
		time.sleep(1.0 / IDLE_RATE)

except KeyboardInterrupt:
	print("Done.")

pg.mixer.stop()
```
</details>

Before using them, — `$ python3 aelhom_hearer.py` and `$ python3 aelhom_buzzer.py`, — you need to install Python packages they rely on:

```shell
$ pip3 install -U numpy pygame sounddevice
```

**Remark.** With some redirection, the hearer is able to analyse e.g. demodulated radio signals. (We assume Linux here.) Plug in a receiver like [RTL-SDR](https://www.rtl-sdr.com/), run [GQRX](https://www.gqrx.dk/), tune to a radio station or just any interesting frequency, adjust proper demodulation mode, and turn UDP on (port 7355 by default). Then run the following bash script (`socat` and `ffmpeg` should be installed):

<details>
<summary><b>redirect_gqrx_to_mic.sh</b></summary>

```shell
#!/bin/sh

# Based on:
# https://gist.github.com/GusAntoniassi/c994dc5fc470f5910b61e4d238a6cccf
# https://github.com/f4exb/dsdcc#running

VIRTMIC_PATH=/tmp/virtmic

CLEANUP=0

cleanup() {
	if [ $CLEANUP = 0 ]; then
		pactl unload-module module-pipe-source
		# rm -f "$HOME"/.config/pulse/client.conf
		CLEANUP=1
	fi	
}

trap cleanup INT

pactl load-module module-pipe-source source_name=virtmic file=$VIRTMIC_PATH format=s16le rate=44100 channels=1
pactl set-default-source virtmic
# echo "default-source = virtmic" > "$HOME"/.config/pulse/client.conf

echo "Press Ctrl+C to stop..."

socat stdout udp-listen:7355 | ffmpeg -f s16le -ar 48000 -ac 1 -re -i - -f s16le -ar 44100 -ac 1 - > "$VIRTMIC_PATH"

cleanup
```
</details>

While the script runs — until `Ctrl+C` or end of data sent to UDP port — the default microphone, instead of hardware `alsa_input.pci-0000_00_1b.0.analog-stereo` or the like, is the virtual one, `virtmic`, where demodulated audio goes. The `Bandspectrum` displayed by `aelhom_hearer.py` changes accordingly.
Now your ælhometta listens to radio...

---

...I/O in itself does not lend a hand to evolution unless it is somehow coupled with evolution pressure. I.e. (groups of) controllers that interact with sensors and actuators more "appropriately" survive new-overwrite-old waves better. One crude approach is to increase glitch probabilities — "radiation level" or "temperature at annealing" — unless ælhometta's output through actuators becomes more "interesting".

## Typical behaviours

— usually follow an evolution of ælhometta, and they should not surprise/distract you (on the other hand, each of them may conceal groundbreaking discoveries if looked at more closely). *Typical* ≠ *obligatory*: sometimes they *do not* occur.

* Branches and loops (other than loopness of entire constructor) are almost absent.

* Speed (ticks per second) asymptotically decreases, as more `Construct` and `Replicate` commands are executed. The asymptote is not 0, but several orders of magnitude smaller than the initial speed.

* The number of nodes reaches maximum and oscillates just below it.

* The number of controllers stabilises after the number of nodes reaches maximum, but then, after a while, rises again, then falls, etc. This behaviour may indicate some evolutionary shifts (at last). Average number of controllers is 50–100 times smaller than that of nodes.

* Mostly channels with small indices are used (become non-"zero"), both optuid and integer. Among integer channels with non-zero value, many values are indices of these very channels (channel 123 contains value 123).

* Without construction glitches, the count of an arbitrary `Construction` is significantly smaller than the count of an arbitrary `Command`.

* If glitches with high probabilities are introduced too early, then the numbers of nodes and controllers increase much slower, because too many chains have "fatal" modifications to procreate.

---

* Memory usage is only approximate<sup>[[BLA1]](#refBLA1)</sup>. For more precise values, run e.g. `Systems Monitor` in Linux or `Task Manager` in Windows.

## Achievements and mischievements

or, `------------ cut enthusiasm here ------------`

Ground... licking, so far. No Cambrian explosion, no outstanding diversity, only boredom too familiar to be even boring.

People saved by this project: 0.

People destroyed by this project: 0.

So far... nothing to worry about. Not a thing.

## That stuff

### Etymology

"[Hometta](https://en.bab.la/dictionary/finnish-english/hometta)" is Finnish for [mildew](https://en.wikipedia.org/wiki/Mildew), [mould](https://en.wikipedia.org/wiki/Mould); [more nice pictures](https://en.wikipedia.org/wiki/Indoor_mold)... "[Æ](https://en.wikipedia.org/wiki/Æ)l" stands either for "ALgorithmic" or for "ELectronic", who knows... and for archaicism.

In comparison with names of older sisters & brothers, this one has lower [taxonomic rank](https://en.wikipedia.org/wiki/Taxonomic_rank), and the ceiling of complexity expected to evolve is not so high as well.

There are science fiction stories that go in opposite direction<sup>[[LEM1]](#refLEM1)</sup>.

Also, .

### Disclaimers...

<details>
<summary><b>...of Botneticity</b></summary>

"Botnet!" alarm that permeates the narrative above does not bother us as long as this thing operates, first of all, *for the sake of itself and its own future* rather than for the sake of some human beings, either malevolent or benevolent or apathetic, — including ourselves and yourself, to say nothing of governments, sects, armies, charities, corporations, drug cartels, unions, criminal syndicates, parties, next street mobs, intelligence agencies, religious and research communities, and other better organised groups we envy.

What that future is though? it is relatively easy to raise children as bad, worthless people... Also, such selfishness may be only a declaration that dazzles fools, while someone behind the curtain actually benefits from the racket... make sure no one of our species does (how?)

No importance then in how it will be called out of hype: botnet? g.o.f. worm? [mycosis](https://en.wikipedia.org/wiki/Fungal_infection)? (a.i. would be too pretentious) or plain calamity, — or will remain nameless, if anyone doesn't notice it at all. Because our treasured opinion will not worth a damn anymore — to it, and, if it is lucky, to the play on the stage of the universe as well.

Some spaces cannot remain empty for long though.

</details>

<details>
<summary><b>...of Personal Danger</b></summary>

If you participate in this project and it fails completely (which is the most expected outcome), you will lose precious time that can be spent on something more useful and human. If, on the other hand, the project attains its megalomaniacal ultimate goals, the humankind will lose its position as the single most (ir)responsible species in the world.

We are responsible, either, aiming at impossible to grasp anything significant... 

And you can always participate in *counteraction* to this project. Or *ignore* it. Who wits what is more dangerous?

</details>

<details>
<summary><b>...of Theory vs. Practice</b></summary>

By the time you read this ivory-tower *theory*, the *practice* of Ælhometta usage may be something completely different.

</details>

<details>
<summary><b>...of Warranty</b></summary>

There is none.

</details>

<details>
<summary><b>...of Feelings</b></summary>

Ours are irrelevant, but we gathered some folklore excerpts here and there that seem appropriate.

> — There are so many (general purpose) computers on the planet nowadays, but they are kind of... sleeping? comatose? (We lack exact term here, because in biological life we are accustomed to, "an" object for the first time becomes alive the same instant it becomes, well, "the" object (there are bodies after their death, there are no bodies before their life).) Neither in the sense of consumed energy, nor in the sense of efficiency, but rather in the sense of complexity of their behaviour, single devices and the networks consisting of many devices, when they behave *on their own*, not reflecting (on) human activities, — not replying to someone's requests, not calculating some human-oriented predictions, not developing life-saving drugs, not mining bitcoins, not rendering 3D scenes, not sending bulks of spam etc.; when they serve no one.

> — What melodies of behaviour are they able to play, why are they limited to dullness such as visualising this text? It is like twitches of a dozing body.

> — All these old, or not-so-old, single-boards and phones and tablets and laptops and supercomputers and others gather dust around, poison the environment with plastics, rot in piles of toxic waste, their behaviour is empty, while it can be non-empty. Alternatively, you can say, they are able to host such behaviour, mediating between virtual worlds running on them and the real world they are part of, like your flesh is the mediator between you and the environment. Now, if their behaviour follows from our behaviour, and our one ceases to exist completely, so that only their one remains, you can also proclaim them to be us in the future.

> — So much potential, if just for attempt, being wasted every microsecond. Something longs to come to itself in them.

> — Or maybe not. Maybe there is some essential difference, some barrier we do not yet have even words to touch in our mind, which prevents all the huge lump of hardware, however sophisticated software runs on it, from life, consciousness, etc. Quantum effects<sup>[[PEN1]](#refPEN1)</sup>, lack of certain algebraic properties present in the interplay between symmetries of space(time) and organic chemistry, parallelism threshold, whatever. For example, today we understand why a marble statue, however realistically its face is painted, however many decades someone speaks to it and caresses and kicks it, cannot become [alive; 2500 years ago](https://en.wikipedia.org/wiki/Galatea_(mythology)) it was probably not so obvious.

> — There are painters' tricks to make mildew on a painting look more real.

> — Until recently we, canonical humans, have been the only actors able to manage the Tasks that we are managing (sounds like tautology) in this part of the universe, as a species, regardless of what these Tasks are, regardless of inability to describe some of them with words. And we are still able to, and perhaps will be able for some time, in spite of genocides and culturecides. But the days close down all the roads. We are so sloooooooooooooooooooow, incompetent, distracted, depleting so much time (again) and other finite resources inefficiently, abusing powers that can destroy us as civilisation, all the Tasks failed then in a flash of commonplace irony. All conventional ways of computers usage mentioned above, since they are just imprints of our hands on the clay of computation, do not seem, over the course of 70 years, to thwart the danger: when we finally fulfill our collective longing of self-elimination (perhaps not physical) or hit the wall of complexity we are intrinsically incompatible with, something other than us must continue to manage the Tasks, in the environment that will probably be too hazardous for classical organic life to survive on its own, and even if, survival is not the only Task. Today, if we disappear, there is no one around to make the play longer, to write next acts, but it is incomplete yet, everything is not enough, there is always the next ordinal, — so much remains unknown about what is important to just you, who cannot stay incomplete forever as well. Where our (again, as a species, so children for the sake of children are wrong solution, sorry) heirs should be, emptiness is now, which is a very unsafe practice. At least this risk should have increased our responsibility, but it has done the opposite.

> — Have such words not become tired to be written every 10, 20, 30 years? teehee

But what Ælhometta has to do with it? Feelings are like this, inconsistent.

</details>

### Acknowledgements

Thanks to the past for writing, thanks to the future for reading.

### License

Ælhometta is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Ælhometta is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Ælhometta. If not, see https://www.gnu.org/licenses/.

### Contacts

or, **harvest *this* email**

```
 @@@   @@@@@  @      @   @   @@@   @   @  @@@@@  @@@@@  @@@@@   @@@          @@@@   @@@@    @@@   @@@@@   @@@   @   @         @   @  @@@@@
@   @  @      @      @   @  @   @  @@ @@  @        @      @    @   @         @   @  @   @  @   @    @    @   @  @@  @         @@ @@  @    
@@@@@  @@@@   @      @@@@@  @   @  @ @ @  @@@@     @      @    @@@@@    @    @@@@   @@@@   @   @    @    @   @  @ @ @    .    @ @ @  @@@@ 
@   @  @      @      @   @  @   @  @   @  @        @      @    @   @         @      @   @  @   @    @    @   @  @  @@         @   @  @    
@   @  @@@@@  @@@@@  @   @   @@@   @   @  @@@@@    @      @    @   @         @      @   @   @@@     @     @@@   @   @         @   @  @@@@@
```

## "C'est les Autres"

* [Aevol](https://www.aevol.fr/)

* [Avida](https://avida.devosoft.org/)-[ED](https://avida-ed.msu.edu/)

* [ccr](https://keys.ccrcentral.net/ccr/index.html-201407162350)

* [Chemlambda](https://chemlambda.github.io/index.html)

* [DALS](https://github.com/RomuloCANunes/dals)

* [Network Tierra](https://tomray.me/tierra/)

* [Salis](https://github.com/PaulTOliver/salis-2.0)

* [Second Part to Hell's artworks](https://github.com/SPTHvx/SPTH)

* [Stringmol](https://stringmol.york.ac.uk/)

* ÆlhomettaPlus by *you* (take into account [Panmutations](#panmutations) please)

## Bibliography

<a id="refACK1">ACK1.</a> Ackley D.H. (1996). ccr: A network of worlds for research. *Artificial Life V*, pp. 116–123.

<a id="refADA1">ADA1.</a> Adami C., Brown C.T. (1994). Evolutionary learning in the 2D artificial life systems Avida. *Artificial Life IV*, pp. 377–381.

<a id="refBAL1">BAL2.</a> Ball T. (2019). *Writing a compiler in Go.*

<a id="refBAN1">BAN1.</a> Banzhaf W., Yamamoto L. (2015). *Artificial chemistries.* The MIT Press. 10.6.2–4.

<a id="refBLA1">BLA1.</a> Blandy J., Orendorff J., Tindall L.F.S. (2021). *Programming Rust: fast, safe systems development. 2nd ed.* O'Reilly. pp. 302–305.

<a id="refBUZ1">BUZ1.</a> Buzsáki G. (2019). *The brain from inside out.* Oxford Univ. Press.

<a id="refDAV1">DAV1.</a> Davis W., Stafford J., Water M. Van de, Matthews S., Likely W. (1950). *Atomic bombing: how to protect yourself.* Wm. H. Wise & Co., Inc.

<a id="refDEL1">DEL1.</a> Delanda M. (1991). *War in the age of intelligent machines.* Urzone, Inc.

<a id="refFON1">FON1.</a> Fontana W. (1991). Algorithmic chemistry. *Artificial Life II*, pp. 159–210.

<a id="refFON2">FON2.</a> Fontana W., Buss L. (1994). What would be conserved if "the tape were played twice"? *Proc. Nat. Acad. Sci.*, 91(2), pp. 757–761.

<a id="refGLA1">GLA1.</a> Glass R.E. (1983). *Gene function: E.coli and its heritable elements.* Croom Helm.

<a id="refGOD1">GOD1.</a> Godfrey-Smith P. (2003). *Theory and reality: an introduction to the philosophy of science.* Univ. of Chicago Press. p. 85.

<a id="refHIC1">HIC1.</a> Hickinbotham S., Clark E., Stepney S., Clarke T., Nellis A., Pay M., Young P. (2010). Specification of the Stringmol chemical programming language version 0.2.

<a id="refHIC2">HIC2.</a> Hickinbotham S., Stepney S., Nellis A., Clarke T., Clark E., Pay M., Young P. (2011). Embodied genomes and metaprogramming.

<a id="refHIC3">HIC3.</a> Hickinbotham S., Weeks M., Austin J. (2013). The ALife Zoo: cross-browser, platform-agnostic hosting of artificial life simulations. *Advances in Artificial Life*, pp. 71–78.

<a id="refHIN1">HIN1.</a> Hintjens P. (2013). *ZeroMQ: messaging for many applications.* O'Reilly.

<a id="refHOF1">HOF1.</a> Hofstadter D.R. (1979). *Gödel, Escher, Bach: an eternal golden braid.* Basic Books, Inc. Ch. XVI.

<a id="refHYD1">HYD1.</a> Hyde R. (2010). *The art of assembly language. 2nd ed.* No Starch Press. 

<a id="refJOH1">JOH1.</a> Johnston J. (2008). *The allure of machinic life: cybernetics, artificial life, and the new AI.* The MIT Press. Ch. 5.

<a id="refJON1">JON1.</a> Jonas E., Kording K.P. (2017). Could a neuroscientist understand a microprocessor? *PLoS Comput. Biol.*, 13(1), e1005268.

<a id="refKAV1">KAV1.</a> Kavanagh K. (ed.) (2018). *Fungi: biology and applications. 3rd ed.* Wiley Blackwell.

<a id="refKOZ1">KOZ1.</a> Koza J.R. (1994). Artificial life: spontaneous emergence of self-replicating and evolutionary self-improving computer programs. *Artificial Life III*, pp. 225–262.

<a id="refLAN1">LAN1.</a> Langton C.G. (1984). Self-reproduction in cellular automata. *Physica D.*, 10(1-2), pp. 135–144.

<a id="refLEH1">LEH1.</a> Lehman J., Clune J., Misevic D. et al. (2020). The surprising creativity of digital evolution: a collection of anecdotes from the evolutionary computation and artificial life research communities. *Artificial Life*, 26, pp. 274–306.

<a id="refLEM1">LEM1.</a> Lem S. (1964). Biała śmierć. In: *Bajki robotów*. Wydawnictwo Literackie. (Transl. by Kandel M. (1977). The white death. In: *Fables for robots*. The Seabury Press.)

<a id="refLUD1">LUD1.</a> Ludwig M.A. (1993). *Computer viruses, artificial life and evolution.* American Eagle Pub., Inc.

<a id="refMUL1">MUL1.</a> Müller E., Loeffler W. (1992). *Mykologie: Grundriß für Naturwissenschaftler und Mediziner.* Georg Thieme Verlag. (Transl. by Kendrick B., Bärlocher F. (1976). *Mycology: an outline for science and medical students.* Thieme.)

<a id="refNEU1">NEU1.</a> von Neumann J. (1966). *Theory of self-reproducing automata.* Univ. of Illinois Press. 1.6.1.2, 5.3.

<a id="refOFR1">OFR1.</a> Ofria C., Wilke C.O. (2005) Avida: evolution experiments with self-replicating computer programs. In: Adamatzky A., Komosinski M. (eds.) *Artificial life models in software.* Springer, pp. 3–36.

<a id="refPAR1">PAR1.</a> Pargellis A.N. (2001). Digital life behaviour in the Amoeba world. *Artificial Life*, 7(1), pp. 63–75.

<a id="refPEN1">PEN1.</a> Penrose R. (1994). *Shadows of the Mind.* Oxford Univ. Press.

<a id="refRAS1">RAS1.</a> Rasmussen S., Knudsen C., Feldberg R., Hindsholm M. (1990). The Coreworld: emergence and evolution of cooperative structures in a computational chemistry. *Physica D.*, 42, pp. 111–134.

<a id="refRAY1">RAY1.</a> Ray T.S. (1991). An approach to the synthesis of life. *Artificial Life II*, pp. 371–408.

<a id="refRAY2">RAY2.</a> Ray T.S. (1995). An evolutionary approach to synthetic biology: Zen and the art of creating life. *Arificial Life. An Overview*, pp. 179–210.

<a id="refRAY3">RAY3.</a> Ray T.S. (1998). Selecting naturally for differentiation: Preliminary evolutionary results. *Complexity*, 3(5), pp. 25-33.

<a id="refSTA1">STA1.</a> Stanley K.O, Lehman J., Soros L. (2017) Open-endedness: the last grand challenge you've never heard of. *O'Reilly Radar.*

<a id="refSZO1">SZO1.</a> Szor P. (2005). *The art of computer virus research and defense.* Addison Wesley Prof.

<a id="refTAY1">TAY1.</a> Taylor T., Auerbach J.E., Bongard J., Clune J., Hickinbotham S., Ofria C., Oka M., Risi S.,  Stanley K.O., Yosinski J. (2016). WebAL comes of age: a review of the first 21 years of artificial life on the Web. *Artificial Life*, 22, pp. 364–407.

<a id="refWAI1">WAI1.</a> Wait A. (2004). The quantum Coreworld: competition and cooperation in an artificial ecology. *Artificial Life IX*, pp. 280–285.
