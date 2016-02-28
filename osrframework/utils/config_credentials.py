# !/usr/bin/python
# -*- coding: cp1252 -*-
#
##################################################################################
#
#    Copyright 2016 Félix Brezo and Yaiza Rubio (i3visio, contacto@i3visio.com)
#
#    This file is part of OSRFramework. You can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
##################################################################################

import ConfigParser
import os
import osrframework.utils.configuration as configuration

def returnListOfCreds():
    '''
        :return:    
            A list of tuples containing in the first the name of the platform, 
            as read from the accounts.cfg file in the application folder. E. g.:
            
            listCreds.append(("<platform>", "<username>", "<password>"))
    '''
    listCreds = []
    # If a accounts.cfg has not been found, creating it by copying from default
    configPath = configuration.getConfigPath("accounts.cfg")

    # Checking if the configuration file exists
    if not os.path.exists(configPath):
        try:
            # Copy the data from the default folder
            defaultConfigPath = configuration.getConfigPath(os.path.join("default", "accounts.cfg"))
     
            with open(configPath, "w") as oF:
                with open(defaultConfigPath) as iF:
                    cont = iF.read()
                    oF.write(cont)        
        except Exception, e:
            print "WARNING. No configuration file could be found and the default file was not found either, so NO credentials have been loaded."
            print str(e)
            print
            return listCreds

    # Reading the configuration file
    config = ConfigParser.ConfigParser()
    config.read(configPath)

    # Iterating through all the sections, which contain the platforms
    for platform in config.sections():
        # Initializing values
        creds = {}
        
        incomplete = False
        
        # Iterating through parametgers
        for (param, value) in config.items(platform):
            if value == '':
                incomplete = True
                break
            creds[param] = value
            
        # Appending credentials if possible
        try:
            if not incomplete:
                listCreds.append((platform, creds["login"], creds["password"]))        
        except:
            pass
    
    return listCreds
