TODO:

* Hyper protocol stack seems to do a lot of what I'm looking for
* And lib p2p seems better supported (by IFPS) and does all the complicated
  network stuff.
* Also, Hyper file systems seem to be similar to the Braid kernel idea.... 

All of these pieces lying around.... how do I package them up into a killer application????

Braid -> State synchronization primitives
State Bus -> Reactive State Synchronization based network
P2P Browser project -> Reactive State Synchronization on top of a BEC broadcast substrate? 
BEC -> https://github.com/automerge/automerge/blob/main/SYNC.md


* Make a blog post describing how to write a twitter clone with this paradigm.
* For the blog post, write a simple key-value storage with a random data generator
  attached to see how it feels



Components:
Public key generator



Problem: Client / Server architecture creates an arbitrary distinction between
turing machines that allows governments, corporations, and etc. to take all of our data.
Solution: Move data onto the client machines. As long as the bits are stored on
    your machine, you can exert choice and power.
How to implement? You need a peer to peer architecture which solves the 
    peer discovery and data serving problems. The system needs to be capable
    of generating app-like experiences (instant sign up, social networking, etc.)

Here's the architecture I'm imagining:
    - Client data store is a simple, heirarchical key-value store, where values are just a byte stream
    - Client is capable of maintaining subscribers, push data as well as pull requests
    - Client throughput is maintained by using an eventually consistent data format (CRDTs?) and lockless concurrency,
      atomic unit of data for streams is a delta
      this protocol is a CRDT's delta
    - Generic DHT engine, which can participate in many DHTs. DHTs become 'community board'
      Unit of identity for this system is (PK, DHT identifier), DHTs can be configured.
    - DHT stores public keys and IP addresses, this IP address is how the physical connection is made (like DNS)
    - Initial peer discovery handled by every DHT being invite only (but anyone can invite)
    - Bit Torrent load balancing enabled in all clients by default, if someone requests data from you
      They automatically also service requests for that data that are seen. 




Basically, the statebus server already provides the entire functionality I need.
in terms of heirarchical key-value storage. What I'm doing new, is designing the 
protocol for inter-server communication
This interserver communication protocol should be able to draw on data from
many different peers and coalesce it into an app-experience (e.g. social media or
organization tools) This system also needs to provide an e2ee mirror protocol so
that people can project their data somewhere more useful (e.g. always on server),
and it needs !!!peer discovery mechanisms!!!, + data location semantics. W h e r e
do likes, comments, and such get stored? Once stored, how do you find their physical
location, given just a public key? 

Public key -> IP address -> Network request
This is the DNS problem. But worse. Already have solution: DHT based DNS. 

Public key turnover / transition?

Easy to do if still access to private key, No solution if lost private key (social-network 
level updates only)

Need CRDTs? Yes. Need to be able to smoothly handle (theoretically) dozens of concurrent
connections, all reading and writing to the same data points. Lockless, eventually consistent
CRDT is _key_. <- This stuff is essential for scaling to internet levels, but inessential for
initial protocol tests. (MAYBE NOT CRDT, BUT DEFINITELY LOCK FREE / EVENTUALLY CONSISTENT)

Read => Run a merge on whatever slice of state is needed
Write => Append to the op log, leave to garbage collection
Stream => Forward raw op logs
Stream-write => Append raw op logs

Statebust model of programming should be expanded to inter-state space paradigm. 
Need to be very careful with eventual protocol design. Backpressure & etc. are vital

Invite only sign up, inviter => Initial peer

BIG IDEA: Use a bittorrent style protocol to generate and spread caches of data for automatic handling of viral posts!!!
Can't have an individual network connection for an individual computer be the only servicer, must delegate or distribute. 
Bake that in to the fundamental.

I'm basically talking about building a parallel internet now, based on an entirely new paradigm. Damn. Good luck me. 
DHT based DNS system, Bittorrent for load balancing, Public key based identity and filtering. Lockless CRDTs as the
fundamnetal data type for easy streaming.

First: Prove the paradigm
Second: Write the protocol package
Third: Design the 

Project steps:
1. Make brain dead simple, file backed, statebus copy-cat (that supports subscriptions) & the public key identity stuff
2. Make the tools to deploy/administer a small cluster of these
3. Write applications for the cluster, learn how the protocol feels, spread the idea.
    a. Hook up Cello to the cluster.
    b. Make a twitter clone on the cluster.

How to deploy an app to my test cluster, P2P? 
Do code distribution via invite-only peer discovery mechanism!!!!! 
Works like this:
1. Write twitter clone app
2. Deploy onto my own instance of the server
3. Publish a share link.
4. People use the link to find my peer, download/replicate the software, and then get added to the peer list
5. Auto load balancing through the bittorrent mechanism, auto initial peer discovery, then use these peers
6. to find other peers, and u have the network traversal mechanism. This means a user's feed is locally cached!!!!

Wow. This is really easy. With a bit-torrent load balancer, easy DHT re-definition, and users running their own database....






So what do we want?

Able to set and get arbitrarily complex values, to any level of specificity, as well as provide data in both
snapshot and stream formats. Needs to be protocol agnostic, so you can write websocket & HTTP & TCP handlers. 
Needs to be able to cryptographically secure the transfer of data end-to-end. Requesters of data must include
public key. Filtering of public keys (also accessible / updateable via the same mechanisms as the rest.)
The instance itself is it's identity. It generates and saves it's private key, and broadcasts it's public key
This instance needs to be able to create arbitrary identity partitions of its self. The data processing layer
needs to have speed bumps attached so that people can't just shove 30gb at you without permission. 

The GUI is itself a client of this arbitrary storage mechanism. 

Public key == identity?


Start with the storage mechanism. Root => This public key. Remote data is namespaced under their public key.
Need to think about how to do cross-namespace data reading with this system. Just grab someone else's public key 
maybe?
 
Think about letting apps/protocols install programs (SANDBOXING!?) to provide richer data processing and retrieval 
 tools? Maybe as before or after hooks on storage? Maybe do it with drivers 🤔

Would love to use diamond types too...