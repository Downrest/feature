# Feature

Personal tool for creating features in my architecture. 

The tool reads a `feature.toml` file which includes references to the preferred paths and services for client/server features.

## Procedure

Before anything, run:
```sh
feature init
```

`feature init` creates a `feature.toml` file which the tool reads into, the syntax is as follows:
```toml
# These are paths to where the features/systems will be created per boundary (client/server). Below are the paths that it defaults to:
# !! NOTE !! It is necessary to have a "/" at the end

client_path = "sync/StarterPlayer/StarterPlayerScripts/GameClient/"
client_service = "StarterPlayer"

server_path = "sync/ServerScriptService/GameServer/"
server_service = "ServerScriptService"
```

Then, run either:
```sh
feature client INSERT_FEATURE_NAME
feature server INSERT_FEATURE_NAME
```

## Example

Example of what is created with this tool:
```sh
feature client Microgame
|
L> sync/StarterPlayer/StarterPlayerScripts/GameClient/Microgame
    L> MicrogameClientController.luau
    L> MicrogameClientHandler.luau
```
```luau
--[[
    MicrogameClientController.luau
]]
local MicrogameClientController = {}

function MicrogameClientController.Init()

end

return MicrogameClientController
```
```luau
--[[
    MicrogameClientHandler.luau
]]
local StarterPlayer = game:GetService("StarterPlayer")

local Test = require(StarterPlayer.StarterPlayerScripts.GameClient.Microgame.MicrogameClientController)

Test.Init()
```