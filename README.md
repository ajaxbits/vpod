# What is this?

This is a way to learn Rust lol

But also, I want a good way to get youtube subscriptions in my podcast player **as audio**

# A plan

- Hook into youtube-dlp
- Stream output to some kind of server that will provide a url
- make an rss feed of those urls
- point podcast app to this rss feed

# Performance

Deciding whether to store the file in memory, or write to disk and do processing

```shell
# hyperfine --prepare 'rm ixrLPGyekCI.m4a' 'curl localhost:3000/download/ixrLPGyekCI'
Benchmark 1: curl localhost:3000/download/ixrLPGyekCI
  Time (mean ± σ):      4.081 s ±  0.179 s    [User: 0.010 s, System: 0.013 s]
  Range (min … max):    3.816 s …  4.355 s    10 runs

# hyperfine 'curl localhost:3000/ixrLPGyekCI'
Benchmark 1: curl localhost:3000/ixrLPGyekCI
  Time (mean ± σ):      2.986 s ±  0.130 s    [User: 0.004 s, System: 0.004 s]
  Range (min … max):    2.753 s …  3.118 s    10 runs
```

So this is the tradeoff: if you return from `stdout`, you are one second faster, but don't get the chapters.

I think this performance could eventually be offset with a type of cache.

# Todo
- [ ] Make the yt-rss part a separate crate
- [ ] Solve the performance problems with getting episode duration [HARD]
- [ ] Think about using `array`s versus `vec`s for the episode lists
- [ ] Change the way you access items from `serde_json` to use `.get()`
- [ ] 

# Implementing new episodes
1. Grab the new rss feed.
2. Grab a new RssFeed item from the file on disk.
3. Use `.truncate()` on the new episodes to eliminate the duplicates
    - Can also use `.retain()` here
    - `.dedup_by_key(F)` [link](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by_key)
    - 
4. 


