extern crate serde_json;

use model::*;

#[test]
fn hufftree() {
	let string = "Testing huffman coding (most efficient single-character lossless text compression) implemented in rust. Here's some random sample text:\
	asdtwadj,srfewkhjnvisht g auihregiwulhstguhlnwysetifdbiuhsfdnvx oahwrgfdxo;hlaywgrfdx rsghfxdb phbregp8 ohuktrohxvc ouhfkb upil4atbr gfdxuiplkb rshofuk 4sroh ubutckyrzex tr 5stdryctv 6diftuyg p7gluibypglu o6f8utvsutrxcf 75exytrcvd6vbf96ogykny80h;oip7t9 glyi75d r6a4 yserxh8o gily8p;ohui fthvb68oguybjhhuk oehfdsnzvclihtnagrfvpgilh5shfilkgsfdibk grfcy5 gsrfxdb kh,jgrsf oxlb,grsfvxce sr8gdvzolhi5wgsfvohkb sgfxhuib sgf h y50hw8ausgrft hgpyujsfdv ogr7jsufv rsgfhvcg olywarsfzv d;h8oiw5 sepfdxh;un tshdxpf hu;nk htd\
".to_string();

	let ht = HuffmanTree::construct(string.clone());
	let mut code = ht.encode(string.clone()).ok().unwrap();

	assert_eq!(Ok(string), ht.decode(&mut code));

	println!("{}", serde_json::to_string_pretty(&ht).unwrap())
}
