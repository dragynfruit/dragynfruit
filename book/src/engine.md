
## WHY?

While Servo *does* have a nice wrapper library (`libservo`), it pulls in *everything*.

We want to pick and choose what's included in Dragynfruit, so we need our own wrapper.

Below are some of our main decisions we've made and our reasoning behind them.

### No WebXR support

WebXR is a pretty niche feature and doesn't make sense for most people.

I don't own any VR or AR devices, so I'm not able to test it either.

