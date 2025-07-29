# Convolution

## Building

```shell
# gui needs to be built
cd gui
npm install
npm run build # or pnpm, bun, etc.
cd ..
cargo xtask bundle convolution_plug --release
```

(Note to self:)
`Get-Content  "C:\Users\<user>\AppData\Local\Bitwig Studio\engine.log" -wait`

### Plans for next iteration

- use (fundsp) Net for DSP
- be able to apply envelope to IRs
- predelay
- decay / speed (something..) - maybe check convology
- reverse (for fun LOL)
- add visual EQ controls (spline type thing)
- support stereo/quad/whatever IRs as well as multiple formats - somewhat supported, but no parameters
- general optimizations
