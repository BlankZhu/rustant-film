# The logos used here are for non-commercial educational purposes.

mkdir -p ./resources/logos

# get apple logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Apple_logo_black.svg/1667px-Apple_logo_black.svg.png -O ./resources/logos/apple.png

# get canon logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/8/8d/Canon_logo.svg/2560px-Canon_logo.svg.png -O ./resources/logos/canon.png

# get fujifilm logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/a/a1/Fujifilm_logo.svg/2560px-Fujifilm_logo.svg.png -O ./resources/logos/fujifilm.png

# get hasselblad logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Hasselblad_logo.svg/2560px-Hasselblad_logo.svg.png -O ./resources/logos/hasselblad.png

# get leica logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/8/8e/Leica_Camera.svg/2048px-Leica_Camera.svg.png -O ./resources/logos/leica.png

# get nikon logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/f/f3/Nikon_Logo.svg/2048px-Nikon_Logo.svg.png -O ./resources/logos/nikon.png

# get olympus logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/2/2d/Olympus_Corporation_logo.svg/2560px-Olympus_Corporation_logo.svg.png -O ./resources/logos/olympus.png

# get om digital solutions logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/5/53/OM_Digital_Solutions_Logo.svg/2560px-OM_Digital_Solutions_Logo.svg.png -O './resources/logos/om ditital solutions.png'

# get lumix logo (instead of panasonic)
wget https://upload.wikimedia.org/wikipedia/commons/thumb/3/3e/Lumix_logo.svg/2560px-Lumix_logo.svg.png -O ./resources/logos/panasonic.png

# get pentax logo...
wget https://upload.wikimedia.org/wikipedia/commons/thumb/8/81/Pentax_Logo.svg/2560px-Pentax_Logo.svg.png -O './resources/logos/pentax.png'

# get phaseone logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/2/28/Phase_One_logo.svg/2560px-Phase_One_logo.svg.png -O './resources/logos/phase one.png'

# get ricoh logo...
wget https://upload.wikimedia.org/wikipedia/commons/thumb/8/8f/Ricoh_logo_2012.svg/2560px-Ricoh_logo_2012.svg.png -O './resources/logos/ricoh.png'

# get sony logo
wget https://upload.wikimedia.org/wikipedia/commons/thumb/c/ca/Sony_logo.svg/2560px-Sony_logo.svg.png -O ./resources/logos/sony.png

# perform convertion
for file in ./resources/logos/*.png; do
    convert "$file" -background white -alpha remove "${file%.png}.jpg"
done

# remove old png
rm ./resources/logos/*.png