<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>button_Create</name>
   <tag></tag>
   <elementGuidId>66ca2239-479b-47f3-bfe3-f62e1353e1bf</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#key-bindings-1</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//button[@id='key-bindings-1']</value>
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
      <webElementGuid>b0b6d114-fb0b-4ab5-a196-7de6317084ee</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:id</name>
      <type>Main</type>
      <value>$id('key-bindings')</value>
      <webElementGuid>3dca83b6-a059-4494-90b8-d9fec832e588</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-mousetrap.global.mod-s</name>
      <type>Main</type>
      <value>document.getElementById($el.id).click()</value>
      <webElementGuid>b0e7627d-3e3b-4e19-9a5f-c1ad3dfc8a57</webElementGuid>
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
      <webElementGuid>928c5b54-a2c1-424b-a9a6-be2ce2f30692</webElementGuid>
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
      <webElementGuid>a8dcdf30-c5e8-4c9f-a730-24919c379282</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:class</name>
      <type>Main</type>
      <value>{ 'enabled:opacity-70 enabled:cursor-wait': isProcessing }</value>
      <webElementGuid>e8a94ba2-c7be-404f-96da-3354758bba66</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>fi-btn relative grid-flow-col items-center justify-center font-semibold outline-none transition duration-75 focus-visible:ring-2 rounded-lg fi-color-custom fi-btn-color-primary fi-color-primary fi-size-md fi-btn-size-md gap-1.5 px-3 py-2 text-sm inline-grid shadow-sm bg-custom-600 text-white hover:bg-custom-500 focus-visible:ring-custom-500/50 dark:bg-custom-500 dark:hover:bg-custom-400 dark:focus-visible:ring-custom-400/50 fi-ac-action fi-ac-btn-action</value>
      <webElementGuid>71e17fd2-335e-47ec-9878-6341474bfc3b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>type</name>
      <type>Main</type>
      <value>submit</value>
      <webElementGuid>bf7b7e19-fb29-49bc-b96b-b4d647833cdf</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>wire:loading.attr</name>
      <type>Main</type>
      <value>disabled</value>
      <webElementGuid>d08b7aac-1809-4072-a22d-d069edf6a91e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:disabled</name>
      <type>Main</type>
      <value>isProcessing</value>
      <webElementGuid>4a9a3fa3-606c-4f73-a492-6d16429bbdad</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>key-bindings-1</value>
      <webElementGuid>4b3c41e6-6878-4aea-8638-76f96cb15413</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-has-alpine-state</name>
      <type>Main</type>
      <value>true</value>
      <webElementGuid>1569d9db-50b0-4a19-a4c4-de21d266402e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            

                    
    
    

        

                    
    
    

        
    

    
        Create
    

            
    

    

    
</value>
      <webElementGuid>6fb180dd-b1e1-49e4-b5e7-3060b3501fc1</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;key-bindings-1&quot;)</value>
      <webElementGuid>98657982-3aba-4b09-9ccc-aa7e088254e9</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//button[@id='key-bindings-1']</value>
      <webElementGuid>f7b35f52-71b0-4ff8-9e1e-56b92be2d431</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='form']/div[2]/div/button</value>
      <webElementGuid>6b7628bc-8ace-4245-a1d1-79f6bf9dfd9a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Judul Gambar'])[1]/following::button[1]</value>
      <webElementGuid>652b067c-98d5-4976-ad0c-a97a5ab2b905</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Files'])[1]/following::button[1]</value>
      <webElementGuid>55fa90df-838d-4e21-b520-b723b0e96fdf</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Create &amp; create another'])[1]/preceding::button[1]</value>
      <webElementGuid>abd76ada-5cb3-4eeb-ab7e-664254b9fd48</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form/div[2]/div/button</value>
      <webElementGuid>f6b13b58-2ab6-43ac-8cc5-4cb48a08ff05</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//button[@type = 'submit' and @id = 'key-bindings-1' and (text() = '
            

                    
    
    

        

                    
    
    

        
    

    
        Create
    

            
    

    

    
' or . = '
            

                    
    
    

        

                    
    
    

        
    

    
        Create
    

            
    

    

    
')]</value>
      <webElementGuid>d0b20e01-3b26-4461-8c7f-7b21367c3547</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
