![It moves](./resources/output.gif)

### TODO:
- [x] guid per websocket
- [x] send in request
- [ ] ping pong to keep child threads alive
- [ ] send heartbeats
- [x] websocket sub on guid
- [x] worker publish to each guild
- [x] worker track guid -> [list of stops requested]
- [x] on disconnect tell worker guid is gone
- [ ] worker check and remove stops that aren't in other guid lists
- [ ] no more heartbeats sent to worker in no guid lists
- [x] graphql api over reference data to lookup trip id from coordinates
- [ ] handle deployments or other worker restarts
    - save to redis and restart tasks on startup?
- [x] check tasks that have died due to error and remove them
    - another background task?
    - or piggy back another call?
- [ ] handle no real time info error?
- [ ] refresh more often for some lines or all?
