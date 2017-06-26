command!(latency(ctx, msg) {
    let latency = ctx.shard.lock()
        .unwrap()
        .latency()
        .map_or_else(|| "N/A".to_owned(), |s| {
            format!("{}.{}s", s.as_secs(), s.subsec_nanos())
        });

    let _ = msg.channel_id.say(&latency);
});
