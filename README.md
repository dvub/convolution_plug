# Convolution

## Downloads

Head over to the Actions tab and go to the latest build and find the artifacts section.

## Building

Please note that only macOS and windows are supported at this time.

```shell
# gui needs to be built
cd gui
npm install
npm run build # or pnpm, bun, etc.
cd ..
cargo xtask bundle convolution_plug --release
```

### Plans for next iteration

- use (fundsp) Net for DSP
- be able to apply envelope to IRs
- predelay
- decay / speed (something..) - maybe check convology
- reverse (for fun LOL)
- add visual EQ controls (spline type thing)
- support stereo/quad/whatever IRs as well as multiple formats - somewhat supported, but no parameters
- general optimizations
