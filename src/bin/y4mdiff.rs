use std::path::PathBuf;

use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Compare y4m files frame by frame
struct Opts {
    /// y4m files to diff
    #[argh(positional)]
    imgs: Vec<PathBuf>,
}

#[derive(PartialEq)]
enum State {
    Equal,
    Different,
}

/// TODO: return the diff
fn cmp(fa: y4m::Frame, fb: y4m::Frame) -> State {
    for (pixa, pixb) in fa.get_y_plane().iter().zip(fb.get_y_plane()) {
        if pixa != pixb {
            return State::Different;
        }
    }
    for (pixa, pixb) in fa.get_u_plane().iter().zip(fb.get_u_plane()) {
        if pixa != pixb {
            return State::Different;
        }
    }
    for (pixa, pixb) in fa.get_v_plane().iter().zip(fb.get_v_plane()) {
        if pixa != pixb {
            return State::Different;
        }
    }

    State::Equal
}

fn main() -> Result<()> {
    let opts: Opts = argh::from_env();

    if opts.imgs.len() != 2 {
        anyhow::bail!("Only 2 y4m files supported right now");
    }

    let mut a = y4m::decode(std::fs::File::open(&opts.imgs[0])?)?;
    let mut b = y4m::decode(std::fs::File::open(&opts.imgs[1])?)?;

    if a.get_width() != b.get_width()
        || a.get_height() != b.get_height()
        || a.get_bit_depth() != b.get_bit_depth()
    //         || a.get_colorspace() != b.get_colorspace()
    {
        anyhow::bail!("Images not comparable");
    }

    let mut count = 0;
    loop {
        let fa = a.read_frame();
        let fb = b.read_frame();

        match (fa, fb) {
            (Ok(fa), Ok(fb)) => {
                if cmp(fa, fb) != State::Equal {
                    eprintln!("Frame {} differ", count);
                }
            }
            (Err(y4m::Error::EOF), Err(y4m::Error::EOF)) => {
                break;
            }
            e => todo!("{:?}", e),
        }

        count += 1;
    }

    Ok(())
}
