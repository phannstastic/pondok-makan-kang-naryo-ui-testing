<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>button_Save changes</name>
   <tag></tag>
   <elementGuidId>ec638ef9-18c1-4ffe-b5a5-c8db0aca5512</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//button[@id='key-bindings-2']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#key-bindings-2</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>button</value>
      <webElementGuid>3d6924a1-8eca-4535-9413-88a594e71f68</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:id</name>
      <type>Main</type>
      <value>$id('key-bindings')</value>
      <webElementGuid>29039733-591b-4790-a924-348b085df93d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-mousetrap.global.mod-s</name>
      <type>Main</type>
      <value>document.getElementById($el.id).click()</value>
      <webElementGuid>90520ce6-1bb0-459d-8dbd-5ce51ef99471</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-data</name>
      <type>Main</type>
      <value>{
            form: null,
            isProcessing: false,
            processingMessage: null,
        }</value>
      <webElementGuid>1ec5eba9-508e-4e12-b019-69493cf5a553</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-init</name>
      <type>Main</type>
      <value>
            form = $el.closest('form')

            form?.addEventListener('form-processing-started', (event) => {
                isProcessing = true
                processingMessage = event.detail.message
            })

            form?.addEventListener('form-processing-finished', () => {
                isProcessing = false
            })
        </value>
      <webElementGuid>a5dc2efd-14ab-481f-93ba-9fff324fa923</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:class</name>
      <type>Main</type>
      <value>{ 'enabled:opacity-70 enabled:cursor-wait': isProcessing }</value>
      <webElementGuid>7ca6e8b9-a6f0-4ee8-b942-3ba7280b75c4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>fi-btn relative grid-flow-col items-center justify-center font-semibold outline-none transition duration-75 focus-visible:ring-2 rounded-lg fi-color-custom fi-btn-color-primary fi-color-primary fi-size-md fi-btn-size-md gap-1.5 px-3 py-2 text-sm inline-grid shadow-sm bg-custom-600 text-white hover:bg-custom-500 focus-visible:ring-custom-500/50 dark:bg-custom-500 dark:hover:bg-custom-400 dark:focus-visible:ring-custom-400/50 fi-ac-action fi-ac-btn-action</value>
      <webElementGuid>f65c6fc4-a257-42a2-b79e-35801a3ef3e0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>type</name>
      <type>Main</type>
      <value>submit</value>
      <webElementGuid>db524028-0481-49d5-927b-9a2c3b1a8d82</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>wire:loading.attr</name>
      <type>Main</type>
      <value>disabled</value>
      <webElementGuid>c16e4a1a-2d13-4d1b-b0dc-7d8bc37091e3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:disabled</name>
      <type>Main</type>
      <value>isProcessing</value>
      <webElementGuid>d059a5b4-481b-4a39-a6b8-179013ce5fea</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>key-bindings-2</value>
      <webElementGuid>1e2b50dd-a80c-4dba-8113-09ba4dbef13e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            

                    
    
    

        

                    
    
    

        
    

    
        Save changes
    

            
    

    

    
</value>
      <webElementGuid>1a67ac3b-5303-4bc0-90e7-52932ab687a6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;key-bindings-2&quot;)</value>
      <webElementGuid>fc550c40-4236-4aa6-93e1-9614fcc11d38</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//button[@id='key-bindings-2']</value>
      <webElementGuid>d6594740-0bea-4ca6-8edf-3c481e4dad1e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='form']/div[2]/div/button</value>
      <webElementGuid>a57d83aa-79cd-4217-a683-69ea5b7f9563</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Files'])[1]/following::button[1]</value>
      <webElementGuid>ba2aa896-a9ed-4789-b1ad-992dcede7f45</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='tap to cancel'])[1]/following::button[1]</value>
      <webElementGuid>bf614e43-aeb9-4fa9-acbe-760787c11659</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Cancel'])[2]/preceding::button[1]</value>
      <webElementGuid>bf4e88f1-1b0d-4e68-93fd-155e958f8b4b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form/div[2]/div/button</value>
      <webElementGuid>f451ae18-1d8e-4c4c-9c34-8e4a08c79830</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//button[@type = 'submit' and @id = 'key-bindings-2' and (text() = '
            

                    
    
    

        

                    
    
    

        
    

    
        Save changes
    

            
    

    

    
' or . = '
            

                    
    
    

        

                    
    
    

        
    

    
        Save changes
    

            
    

    

    
')]</value>
      <webElementGuid>887c9750-1338-4b9b-bcd5-57b3333fab33</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
