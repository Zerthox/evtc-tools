# EVTC Tools
Extract information from ArcDPS EVTC log files.

## Usage
```sh
# extract all events
evtc_tools all path/to/log.zevtc

# filter events for specific source/dest agent
evtc_tools all log.zevtc --agent 2000 # using arcdps id
evtc_tools all log.zevtc --agent inst:123 # using instance id
evtc_tools all log.zevtc --agent "Agent Name"
evtc_tools all log.zevtc --target "Target Name"

# extract agents
evtc_tools agents log.zevtc

# extract skill/buff information with optional filter
evtc_tools skills log.zevtc
evtc_tools skills log.zevtc --skill 123456
evtc_tools skills log.zevtc --skill "Skill/Buff Name"

# extract casts with optional filter
evtc_tools casts log.zevtc
evtc_tools casts log.zevtc --skill 123456
evtc_tools casts log.zevtc --skill "Skill Name"

# extract hit to weapon set mapping
evtc_tools hitmap log.zevtc --agent 2000

# extract pov gear information
evtc_tools gear log.zevtc

# extract position data
evtc_tools positions log.zevtc --agent 2000

# extract (visual) effect data
evtc_tools effects log.zevtc

# extract missile (projectile) data
evtc_tools missiles log.zevtc
```
