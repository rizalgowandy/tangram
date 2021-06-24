use anyhow::Result;
use clap::Clap;
use std::path::Path;
use tangram_build::{Target, TargetFileNames};

#[derive(Clap)]
pub struct Args {}

pub fn run(_args: Args) {
	let tangram_path = std::env::current_dir().unwrap();
	let dist_path = tangram_path.join("dist");

	eprintln!("elixir");
	let elixir_priv_path = tangram_path.join("languages/elixir/priv");
	if std::fs::metadata(&elixir_priv_path)
		.map(|m| m.is_dir())
		.unwrap_or(false)
	{
		std::fs::remove_dir_all(&elixir_priv_path).unwrap();
	}
	for target in [
		Target::X8664UnknownLinuxGnu,
		Target::AArch64UnknownLinuxGnu,
		Target::X8664UnknownLinuxMusl,
		Target::AArch64UnknownLinuxMusl,
		Target::X8664AppleDarwin,
		Target::AArch64AppleDarwin,
		Target::X8664PcWindowsMsvc,
	] {
		let target_file_names = TargetFileNames::for_target(target);
		install(
			&dist_path
				.join(target.as_str())
				.join(target_file_names.tangram_elixir_src_file_name),
			&elixir_priv_path
				.join(target.as_str())
				.join(target_file_names.tangram_elixir_dst_file_name),
		)
		.unwrap();
	}

	eprintln!("go");
	let go_libtangram_path = tangram_path.join("languages/go/libtangram");
	if std::fs::metadata(&go_libtangram_path)
		.map(|m| m.is_dir())
		.unwrap_or(false)
	{
		std::fs::remove_dir_all(&go_libtangram_path).unwrap();
	}
	for target in [
		Target::X8664UnknownLinuxMusl,
		Target::AArch64UnknownLinuxMusl,
		Target::X8664AppleDarwin,
		Target::AArch64AppleDarwin,
		Target::X8664PcWindowsGnu,
	] {
		let target_file_names = TargetFileNames::for_target(target);
		install(
			&dist_path
				.join(target.as_str())
				.join(target_file_names.tangram_h_file_name),
			&go_libtangram_path
				.join(target.as_str())
				.join(target_file_names.tangram_h_file_name),
		)
		.unwrap();
		install(
			&dist_path
				.join(target.as_str())
				.join(target_file_names.libtangram_static_file_name),
			&go_libtangram_path
				.join(target.as_str())
				.join(target_file_names.libtangram_static_file_name),
		)
		.unwrap();
	}

	eprintln!("node");
	let node_dist_path = tangram_path.join("languages/node/native");
	if std::fs::metadata(&node_dist_path)
		.map(|m| m.is_dir())
		.unwrap_or(false)
	{
		std::fs::remove_dir_all(&node_dist_path).unwrap();
	}
	for target in [
		Target::X8664UnknownLinuxGnu,
		Target::AArch64UnknownLinuxGnu,
		Target::X8664UnknownLinuxMusl,
		Target::AArch64UnknownLinuxMusl,
		Target::X8664AppleDarwin,
		Target::AArch64AppleDarwin,
		Target::X8664PcWindowsMsvc,
	] {
		let target_file_names = TargetFileNames::for_target(target);
		install(
			&dist_path
				.join(target.as_str())
				.join(target_file_names.tangram_node_src_file_name),
			&node_dist_path
				.join(target.as_str())
				.join(target_file_names.tangram_node_dst_file_name),
		)
		.unwrap();
	}

	eprintln!("python");
	let python_dist_path = tangram_path.join("languages/python/dist");
	if std::fs::metadata(&python_dist_path)
		.map(|m| m.is_dir())
		.unwrap_or(false)
	{
		std::fs::remove_dir_all(&python_dist_path).unwrap();
	}
	for target in [
		Target::X8664UnknownLinuxGnu,
		Target::AArch64UnknownLinuxGnu,
		Target::X8664AppleDarwin,
		Target::AArch64AppleDarwin,
		Target::X8664PcWindowsMsvc,
	] {
		let dist_target_path = dist_path.join(target.as_str());
		for entry in std::fs::read_dir(dist_target_path).unwrap() {
			let path = entry.unwrap().path();
			let is_wheel = path
				.extension()
				.and_then(|e| e.to_str())
				.map(|e| e == "whl")
				.unwrap_or(false);
			if is_wheel {
				install(&path, &python_dist_path.join(path.file_name().unwrap())).unwrap();
			}
		}
	}

	eprintln!("ruby");
	let ruby_libtangram_path = tangram_path.join("languages/ruby/lib/tangram/libtangram");
	if std::fs::metadata(&ruby_libtangram_path)
		.map(|m| m.is_dir())
		.unwrap_or(false)
	{
		std::fs::remove_dir_all(&ruby_libtangram_path).unwrap();
	}
	for target in [
		Target::X8664UnknownLinuxGnu,
		Target::AArch64UnknownLinuxGnu,
		Target::X8664UnknownLinuxMusl,
		Target::AArch64UnknownLinuxMusl,
		Target::X8664AppleDarwin,
		Target::AArch64AppleDarwin,
		Target::X8664PcWindowsMsvc,
	] {
		let target_file_names = TargetFileNames::for_target(target);
		install(
			&dist_path
				.join(target.as_str())
				.join(target_file_names.libtangram_dynamic_file_name),
			&ruby_libtangram_path
				.join(target.as_str())
				.join(target_file_names.libtangram_dynamic_file_name),
		)
		.unwrap();
	}
}

fn install(src: &Path, dst: &Path) -> Result<()> {
	std::fs::create_dir_all(dst.parent().unwrap())?;
	std::fs::copy(src, dst)?;
	Ok(())
}
