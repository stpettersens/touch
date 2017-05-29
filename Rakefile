require 'os'

target = "touch"
tp = "target/release/touch"

if OS.windows? then
    target = "touch.exe"
    tp = "target\\release\\touch.exe"
end

task :default do
    sh "cargo build --release"
end

task :upx => [:default] do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "upx -9 #{tp} -o #{target}"
end

task :clean do
    sh "cargo clean" 
end

task :cleanlock do
    File.delete("Cargo.lock")
end

task :test do
    sh "#{tp} --help"
    puts
    sh "#{tp} foo.txt"
    puts
    if not OS.windows? then
        sh "file foo.txt"
    else
        sh "dir"
    end
end

task :shouldfail do
    sh "#{tp} bar.txt -d invalid"
end
