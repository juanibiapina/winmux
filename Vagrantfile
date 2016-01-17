# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/trusty64"

  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install xorg-dev

    curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --yes
  SHELL
end
