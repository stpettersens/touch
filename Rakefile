require 'fileutils'
require 'os'

target = "touch"

task :default do
    sh "cargo build --release"
end

task :upx => [:default] do
    if OS.windows? then
        if File.exists?("#{target}.exe") then
            File.delete("#{target}.exe")
        end
        sh "upx -9 target\\release\\#{target}.exe -o #{target}.exe"
    else
        if File.exists?(target) then
            File.delete(target)
        end
        sh "upx -9 target/release/#{target} -o #{target}"
    end
end

task :clean do
    FileUtils.rm_rf("target")
    if OS.windows? and File.exists?("#{target}.exe") then
        File.delete("#{target}.exe")
    elsif File.exists?(target) then
        File.delete(target)
    end
end

task :cleanlock do
    File.delete("Cargo.lock")
end

task :test do
    if OS.windows? then
        sh "target\\release\\#{target}.exe --help"
        puts
        sh "target\\release\\#{target}.exe foo.txt"
    else
        sh "target/release/#{target} --help"
        puts
        sh "target/release/#{target} foo.txt"
        sh "file foo.txt"
    end
end
