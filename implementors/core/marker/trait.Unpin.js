(function() {var implementors = {};
implementors["avmath"] = [{"text":"impl Unpin for Layer","synthetic":true,"types":[]},{"text":"impl Unpin for GeometricAltitude","synthetic":true,"types":[]},{"text":"impl Unpin for GeopotentialAltitude","synthetic":true,"types":[]},{"text":"impl Unpin for PressureAltitude","synthetic":true,"types":[]},{"text":"impl Unpin for DensityAltitude","synthetic":true,"types":[]},{"text":"impl Unpin for AltimeterSetting","synthetic":true,"types":[]}];
implementors["gauge_sys"] = [{"text":"impl Unpin for FsContext","synthetic":true,"types":[]},{"text":"impl Unpin for RawServiceId","synthetic":true,"types":[]},{"text":"impl Unpin for GaugeDrawData","synthetic":true,"types":[]},{"text":"impl Unpin for RawUnit","synthetic":true,"types":[]},{"text":"impl Unpin for RawAircraftVariable","synthetic":true,"types":[]},{"text":"impl Unpin for RawNamedVariable","synthetic":true,"types":[]},{"text":"impl Unpin for ServiceId","synthetic":true,"types":[]}];
implementors["simconnect_sys"] = [{"text":"impl Unpin for SimConnect","synthetic":true,"types":[]},{"text":"impl Unpin for DataDefinition","synthetic":true,"types":[]},{"text":"impl&lt;Group&gt; Unpin for NotificationGroupDefinition&lt;Group&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Group: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;EventType&gt; Unpin for EventDefinition&lt;EventType&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;EventType: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for ReceiveHeader","synthetic":true,"types":[]},{"text":"impl Unpin for ReceiveOpen","synthetic":true,"types":[]},{"text":"impl Unpin for Version","synthetic":true,"types":[]},{"text":"impl Unpin for ReceiveEvent","synthetic":true,"types":[]},{"text":"impl Unpin for RawDataDefinitionId","synthetic":true,"types":[]},{"text":"impl Unpin for RawObjectId","synthetic":true,"types":[]},{"text":"impl Unpin for RawNotificationGroupId","synthetic":true,"types":[]},{"text":"impl Unpin for RawEventId","synthetic":true,"types":[]},{"text":"impl Unpin for NotificationGroupPriority","synthetic":true,"types":[]},{"text":"impl Unpin for RawDataSetFlag","synthetic":true,"types":[]},{"text":"impl Unpin for RawMessageType","synthetic":true,"types":[]},{"text":"impl Unpin for RawDataType","synthetic":true,"types":[]},{"text":"impl Unpin for SimConnectHandle","synthetic":true,"types":[]},{"text":"impl Unpin for Handle","synthetic":true,"types":[]},{"text":"impl Unpin for WindowHandle","synthetic":true,"types":[]},{"text":"impl Unpin for HResult","synthetic":true,"types":[]},{"text":"impl Unpin for DataSetFlag","synthetic":true,"types":[]},{"text":"impl Unpin for MessageType","synthetic":true,"types":[]},{"text":"impl Unpin for DataType","synthetic":true,"types":[]}];
implementors["wt_cj4"] = [{"text":"impl Unpin for FadecController","synthetic":true,"types":[]},{"text":"impl Unpin for Aircraft","synthetic":true,"types":[]},{"text":"impl Unpin for Engine","synthetic":true,"types":[]},{"text":"impl Unpin for EngineReadings","synthetic":true,"types":[]},{"text":"impl Unpin for Environment","synthetic":true,"types":[]},{"text":"impl Unpin for Instruments","synthetic":true,"types":[]},{"text":"impl Unpin for Snapshot","synthetic":true,"types":[]},{"text":"impl Unpin for ThrottleAxis","synthetic":true,"types":[]},{"text":"impl Unpin for ThrustValue","synthetic":true,"types":[]},{"text":"impl Unpin for ThrottlePercent","synthetic":true,"types":[]},{"text":"impl Unpin for ThrottleMode","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for EngineData&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for EngineNumber","synthetic":true,"types":[]}];
implementors["wt_flight_recorder"] = [{"text":"impl&lt;T&gt; Unpin for FlightDataRecorder&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["wt_flight_to_csv"] = [{"text":"impl Unpin for FlatSnapshot","synthetic":true,"types":[]},{"text":"impl Unpin for Loop","synthetic":true,"types":[]}];
implementors["wt_systems"] = [{"text":"impl Unpin for PidComponents","synthetic":true,"types":[]},{"text":"impl&lt;In&gt; !Unpin for PidConfiguration&lt;In&gt;","synthetic":true,"types":[]},{"text":"impl&lt;In&gt; !Unpin for PidController&lt;In&gt;","synthetic":true,"types":[]},{"text":"impl&lt;In&gt; !Unpin for PidConfiguration&lt;In&gt;","synthetic":true,"types":[]},{"text":"impl&lt;In&gt; !Unpin for PidController&lt;In&gt;","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()