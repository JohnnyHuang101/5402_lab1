
use std::sync::atomic::AtomicBool;

type Play = Vec<(usize,String,String)>; // line#, player, line

const MIN_ARGS: usize =2;
const MAX_ARGS: usize =3; // includes extra complaints

const PROGRAM_NAME: usize =0;
const CONFIG_FILE: usize =1;
const WHINGE_STATUS: usize =2;

const BAD_ARGS: u8 = 8;
const BAD_COMMAND_LINE_ARGS: u8 = 1;
const GENERATION_FAILURE: u8 = 1;

static WHINGE: AtomicBool = AtomicBool::new(false);
