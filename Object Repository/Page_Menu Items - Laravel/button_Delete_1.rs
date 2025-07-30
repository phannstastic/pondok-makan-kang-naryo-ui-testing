<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>button_Delete_1</name>
   <tag></tag>
   <elementGuidId>9c9c3319-ca48-4420-ba74-6d8e689fdced</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(//button[@type='submit'])[2]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value></value>
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
      <webElementGuid>c3f6c2d1-37b7-464b-8ae0-d804f7ac182d</webElementGuid>
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
      <webElementGuid>c06b7b0e-f92c-4228-8b0f-4c87ca77cf84</webElementGuid>
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
      <webElementGuid>6fe1e500-31a3-4daf-8eae-c44cd6b0cc16</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:class</name>
      <type>Main</type>
      <value>{ 'enabled:opacity-70 enabled:cursor-wait': isProcessing }</value>
      <webElementGuid>3b241b3f-31e5-4b93-9b8b-2cd5d5427312</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>fi-btn relative grid-flow-col items-center justify-center font-semibold outline-none transition duration-75 focus-visible:ring-2 rounded-lg fi-color-custom fi-btn-color-danger fi-color-danger fi-size-md fi-btn-size-md gap-1.5 px-3 py-2 text-sm inline-grid shadow-sm bg-custom-600 text-white hover:bg-custom-500 focus-visible:ring-custom-500/50 dark:bg-custom-500 dark:hover:bg-custom-400 dark:focus-visible:ring-custom-400/50 fi-ac-action fi-ac-btn-action</value>
      <webElementGuid>403c8501-7212-4009-944d-0163ae09b341</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>type</name>
      <type>Main</type>
      <value>submit</value>
      <webElementGuid>0360c07b-7ead-41ff-8ee2-47b915c39e8b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>wire:loading.attr</name>
      <type>Main</type>
      <value>disabled</value>
      <webElementGuid>c058f726-34bd-4947-87e4-88b147f78d36</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:disabled</name>
      <type>Main</type>
      <value>isProcessing</value>
      <webElementGuid>8ca4602d-0fa7-4980-9b62-81a23ad9f181</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            

                    
    
    

        

                    
    
    

        
    

    
        Delete
    

            
    

    

    
</value>
      <webElementGuid>51ebb9da-9303-4ea3-b159-b6bc093ca68f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;fi min-h-screen&quot;]/body[@class=&quot;fi-body fi-panel-admin min-h-screen bg-gray-50 font-normal text-gray-950 antialiased dark:bg-gray-950 dark:text-white&quot;]/div[@class=&quot;fi-layout flex min-h-screen w-full flex-row-reverse overflow-x-clip&quot;]/div[@class=&quot;fi-main-ctn w-screen flex-1 flex-col opacity-0&quot;]/main[@class=&quot;fi-main mx-auto h-full w-full px-4 md:px-6 lg:px-8 max-w-7xl&quot;]/div[@class=&quot;fi-page fi-resource-list-records-page fi-resource-menu-items&quot;]/section[@class=&quot;flex flex-col gap-y-8 py-8&quot;]/div[1]/div[@class=&quot;grid flex-1 auto-cols-fr gap-y-8&quot;]/div[@class=&quot;flex flex-col gap-y-6&quot;]/div[@class=&quot;fi-ta&quot;]/form[2]/div[@class=&quot;fi-modal block fi-modal-open&quot;]/div[1]/div[@class=&quot;fixed inset-0 z-40 overflow-y-auto cursor-pointer&quot;]/div[@class=&quot;relative grid min-h-full grid-rows-[1fr_auto_1fr] justify-items-center sm:grid-rows-[1fr_auto_3fr] p-4&quot;]/div[@class=&quot;fi-modal-window pointer-events-auto relative row-start-2 flex w-full cursor-default flex-col bg-white shadow-xl ring-1 ring-gray-950/5 dark:bg-gray-900 dark:ring-white/10 mx-auto rounded-xl max-w-md&quot;]/div[@class=&quot;fi-modal-footer w-full px-6 pb-6 mt-6&quot;]/div[@class=&quot;fi-modal-footer-actions gap-3 flex flex-col-reverse sm:grid sm:grid-cols-[repeat(auto-fit,minmax(0,1fr))]&quot;]/button[@class=&quot;fi-btn relative grid-flow-col items-center justify-center font-semibold outline-none transition duration-75 focus-visible:ring-2 rounded-lg fi-color-custom fi-btn-color-danger fi-color-danger fi-size-md fi-btn-size-md gap-1.5 px-3 py-2 text-sm inline-grid shadow-sm bg-custom-600 text-white hover:bg-custom-500 focus-visible:ring-custom-500/50 dark:bg-custom-500 dark:hover:bg-custom-400 dark:focus-visible:ring-custom-400/50 fi-ac-action fi-ac-btn-action&quot;]</value>
      <webElementGuid>5c0ea79b-8c7d-4b38-a19f-f77edab961e3</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>(//button[@type='submit'])[2]</value>
      <webElementGuid>53b00dce-df8e-47d2-8612-86076a689ea2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Cancel'])[1]/following::button[1]</value>
      <webElementGuid>254b6ff3-6a49-46d8-a4a8-df7c13d6f8a0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Delete menu item'])[1]/following::button[2]</value>
      <webElementGuid>f72aaf3a-9dbf-4adb-a817-36698695ae1f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Laravel'])[1]/preceding::button[1]</value>
      <webElementGuid>8b453928-2bcf-49d3-96f1-b0024fffbae5</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/div/div[2]/div/button[2]</value>
      <webElementGuid>f6717989-5925-476e-81ee-4063cedc03d6</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//button[@type = 'submit' and (text() = '
            

                    
    
    

        

                    
    
    

        
    

    
        Delete
    

            
    

    

    
' or . = '
            

                    
    
    

        

                    
    
    

        
    

    
        Delete
    

            
    

    

    
')]</value>
      <webElementGuid>807bd37b-0bf3-4cdc-a457-155daa363370</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
