# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/trusty64"

  config.vm.provider "virtualbox" do |v|
    v.gui = true
  end

  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install xorg
    apt-get install xorg-dev

    curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --yes
  SHELL
end
