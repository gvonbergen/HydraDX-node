
#!/usr/bin/env bash
# Original code from https://github.com/paritytech/polkadot

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

# Find the current version from Cargo.toml
VERSION=`grep "^# v" ./CHANGELOG.md | egrep -o "([0-9\.]+)" | head -n 1`
GITUSER=gvonbergen
GITREPO=hydradx-node

# Build the image
echo "Building ${GITUSER}/${GITREPO}:latest docker image, hang on!"
time docker build -f ./docker/Dockerfile --build-arg PROFILE=release -t ${GITUSER}/${GITREPO}:latest .

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${GITREPO}

echo -e "\nIf you just built version ${VERSION}, you may want to update your tag:"
echo " $ docker tag ${GITUSER}/${GITREPO}:latest ${GITUSER}/${GITREPO}:${VERSION}"

popd