# ArcDPS Log Tools
Extract information from ArcDPS EVTC log files.

## Usage
You can download available [builds from GitHub Actions](https://github.com/Zerthox/arcdps-log-tools/actions/workflows/build.yml) or manually build from source using [Rust](https://www.rust-lang.org/learn/get-started).

```sh
# extract all events
arcdps_log_tools all path/to/log.zevtc

# filter events for specific source/dest agent
arcdps_log_tools all log.zevtc --agent 2000 # using arcdps id
arcdps_log_tools all log.zevtc --agent inst:123 # using instance id
arcdps_log_tools all log.zevtc --agent "Agent Name"
arcdps_log_tools all log.zevtc --target "Target Name"

# extract skill/buff information with optional filter
arcdps_log_tools skill log.zevtc
arcdps_log_tools skill log.zevtc --skill 123456
arcdps_log_tools skill log.zevtc --skill "Skill/Buff Name"

# extract casts with optional filter
arcdps_log_tools cast log.zevtc
arcdps_log_tools cast log.zevtc --skill 123456
arcdps_log_tools cast log.zevtc --skill "Skill Name"

# extract position data
arcdps_log_tools position log.zevtc --agent 2000
```
