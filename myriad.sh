#!/usr/bin/env bash

set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT/.maintain/kubernetes

echo "Deploying"
helm upgrade myriad-node . \
    --install \
    --set-string image.tag=latest-alpha \
    --set-string config.chain=development \
    --set-string nodes[0].name=full1 \
    --set nodes[0].bootnode=true \
    --set nodes[0].validator=false \
    --set-string nodes[0].keys.private=0e78a2628a96c8427c35494b8e2aec8a9387c1e63df98e96aa33cc6835f664e6 \
    --set-string nodes[0].keys.public=12D3KooWRR59MDPGNdtS3JwkyMfPtYZD1NjQKYWmoZDReGDvdzhM \
    --set nodes[0].sessionInjectionEnabled=false \
    --set-string nodes[1].name=full2 \
    --set nodes[1].bootnode=true \
    --set nodes[1].validator=false \
    --set-string nodes[1].keys.private=bcca22c167a5e9dbfa13dc93216e2a5c154573755a089fdff53735333f6d8ca5 \
    --set-string nodes[1].keys.public=12D3KooWPE8raK4UznRdEMQb27sfbbmhywZko34LsMeZxvzH2hNo \
    --set nodes[1].sessionInjectionEnabled=false \
    --set-string nodes[2].name=validator1 \
    --set nodes[2].bootnode=true \
    --set nodes[2].validator=true \
    --set-string nodes[2].keys.private=9caf9ea3cad0ffb3f0f504415e87433fe1bc3047a24ee45b8cbdd89f1f37ed99 \
    --set-string nodes[2].keys.public=12D3KooWFBbga2LJe5FqUdHJ3Y3uqUKGjKBvdi7fPeZhNWbfqbdx \
    --set nodes[2].sessionInjectionEnabled=true \
    --set-string nodes[2].sessionKeys.grandpa=0xdfb839beaf6fe750ca87b9059161d43f2682a6c3a0ac765f1e5054063ed9903b \
    --set-string nodes[2].sessionKeys.babe=0xd811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073 \
    --set-string nodes[2].sessionKeys.imonline=0xd811839e01e3cc6eeb64e6f312a1eaf2988ae2c5fea9dd0b8ac018c146ca7073 \
    --set-string nodes[2].sessionKeys.beefy=0x02d337069cb73bcefafc4e35e5189ad62932e4f2ee3f985b6bbff654cb68017ff1 \
    --set-string nodes[2].sessionKeys.keySeed="fall deal book genuine tonight chimney angry steak proof wheel bag faith//1" \
    --set-string nodes[3].name=validator2 \
    --set nodes[3].bootnode=true \
    --set nodes[3].validator=true \
    --set-string nodes[3].keys.private=69cf93a022112af57036e4a6613f6ecc63218a59fc863962efbd133cb58a7a07 \
    --set-string nodes[3].keys.public=12D3KooWApJunKkvE75fHSBjaZdQhAtomnDTaRoGVnrJ4CXeQxXS \
    --set nodes[3].sessionInjectionEnabled=true \
    --set-string nodes[3].sessionKeys.grandpa=0x6422ce120d8acb2fe261be3b230e0e51c29228bf0075db160b5e3c5455c012c5 \
    --set-string nodes[3].sessionKeys.babe=0x6a95359ecc0e8ae0cb8396f6e21fba4448ba5a0003ee1e0322352a4d8ba3213f \
    --set-string nodes[3].sessionKeys.imonline=0x6a95359ecc0e8ae0cb8396f6e21fba4448ba5a0003ee1e0322352a4d8ba3213f \
    --set-string nodes[3].sessionKeys.beefy=0x032019159fdae5d7f0620f0e5f7c9b382b2f74f047c0162c59732b4770897a2bb7 \
    --set-string nodes[3].sessionKeys.keySeed="fall deal book genuine tonight chimney angry steak proof wheel bag faith//2"

popd
