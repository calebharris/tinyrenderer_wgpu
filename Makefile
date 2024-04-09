# variables for the top-level organization of the repo
# := means simple assignment, values on the right won't be further expanded
app_dir := ./app
site_dir := ./docs

bin_name := tinyrenderer_wgpu

# function that recursively scans a directory tree for files matching a given
# pattern; copied from
# https://stackoverflow.com/questions/2483182/recursive-wildcards-in-gnu-make
rwildcard = $(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

# use rwildcard to build lists of files that, when changed, should trigger a
# rebuild of the rust binaries and wasm packages
# = means recursive assignment, values on the right will be further expanded
app_sources = $(call rwildcard, $(app_dir)/src, *.rs)
app_resources = $(call rwildcard, $(app_dir)/res, *)

wasm_dir = $(site_dir)/assets
wasm_files = $(wasm_dir)/$(bin_name)_bg.wasm \
             $(wasm_dir)/$(bin_name)_bg.wasm.d.ts \
			 $(wasm_dir)/$(bin_name).d.ts \
			 $(wasm_dir)/$(bin_name).js

# if we're not building with the release profile, we don't need to pass a flag
# to cargo, so set this variable to nothing by default
rs_prof_flag :=

# if we are building the release target, set the variable accordingly
release: rs_prof_flag := -r

# this is necessary to enable a second phase of variable expansion, which is
# required to use automatic variables in the prerequisites list; see here for
# more info:
# https://www.gnu.org/software/make/manual/html_node/Secondary-Expansion.html
.SECONDEXPANSION :
profiles := debug release

# this creates two rules that relate the debug and release targets to the file
# locations of the respective binaries that cargo builds
$(profiles) : $(app_dir)/target/$$@/$(bin_name)

# a pattern rule that attaches the same list of prerequisite files to both the
# release and debug binaries
./app/target/%/$(bin_name) : $(rs_sources) $(resources)
	cd app && cargo build $(rs_prof_flag)

wasm : $(wasm_files)

$(wasm_files) : $(app_sources)
	cd $(app_dir) && \
	wasm-pack build --target web --no-pack --out-dir ../$(wasm_dir)
	rm $(wasm_dir)/.gitignore

.PHONY : clean

clean : clean-app clean-site

clean-app : clean-bin clean-wasm

clean-bin :
	cd app && cargo clean

clean-wasm :
	rm $(wasm_files)

clean-site :
	cd $(site_dir) && bundle exec jekyll clean
