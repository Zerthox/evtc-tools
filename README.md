# EVTC Tools
Extract information from ArcDPS EVTC log files.

## Usage
You can download available [builds from GitHub Actions](https://github.com/Zerthox/arcdps-log-tools/actions/workflows/build.yml) or manually build from source using [Rust](https://www.rust-lang.org/learn/get-started).

```sh
# extract all events
evtc_tools all path/to/log.zevtc

# filter events for specific source/dest agent
evtc_tools all log.zevtc --agent 2000 # using arcdps id
evtc_tools all log.zevtc --agent inst:123 # using instance id
evtc_tools all log.zevtc --agent "Agent Name"
evtc_tools all log.zevtc --target "Target Name"

# extract skill/buff information with optional filter
evtc_tools skill log.zevtc
evtc_tools skill log.zevtc --skill 123456
evtc_tools skill log.zevtc --skill "Skill/Buff Name"

# extract casts with optional filter
evtc_tools cast log.zevtc
evtc_tools cast log.zevtc --skill 123456
evtc_tools cast log.zevtc --skill "Skill Name"

# extract hit to weapon set mapping
evtc_tools hitmap log.zevtc --agent 2000

# extract position data
evtc_tools position log.zevtc --agent 2000

# extract (visual) effect data
evtc_tools effect log.zevtc
```
