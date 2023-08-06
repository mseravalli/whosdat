#!/opt/homebrew/bin/bash

pushd /tmp
mkdir -p staging
rm -rf staging/*
pushd staging

mkdir -p partners

gkk_team="/tmp/gkk-team.html";
curl https://www.gkkpartners.de/firm/team.html > ${gkk_team}

for pic in $(cat ${gkk_team} | rg "src=\"(files.*media.*jp[e]?g)\"" -o -r '$1'); do
  wget "https://www.gkkpartners.de/${pic}"
done

for img in *.jpeg; do new_name=$(echo ${img} | sed "s@jpeg@jpg@g")       ; mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@ @_@g")            ; mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@_\.@.@g")          ; mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@_[1-9]@@g")        ; mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@_[mM]osaik@@g")    ; mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@_[oO]rig@@g")      ; mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@_[kK]opie@@g")     ; mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@_[nN]eu@@g")       ; mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@_[pP]ortrait@@g")  ; mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@_[bB]earbeitet@@g"); mv "${img}" "${new_name}" ; done
for img in *.jpg;  do new_name=$(echo ${img} | sed "s@[1-9]@@g")         ; mv "${img}" "${new_name}" ; done

# for img in *.jpg;  do new_name=$(echo ${img} | sed "s@_[a-b]@@g")        ; mv "${img}" "${new_name}" ; done

readarray -t partners_names < <(cat ${gkk_team} | rg "<div>Partner" -B 7 | rg "src=\"(files.*media.*jp[e]?g)\"" -o -r '$1' | rg 'mosaik/(\w+)_(\w+)_' -o -r '${1}_${2}')
for p in "${partners_names[@]}"; do
  ls | rg ${p} | xargs -I {} cp {} partners/{}
done


popd
popd
