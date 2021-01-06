(function() {var implementors = {};
implementors["avmath"] = [{"text":"impl Copy for GeometricAltitude","synthetic":false,"types":[]},{"text":"impl Copy for GeopotentialAltitude","synthetic":false,"types":[]},{"text":"impl Copy for PressureAltitude","synthetic":false,"types":[]},{"text":"impl Copy for DensityAltitude","synthetic":false,"types":[]},{"text":"impl Copy for AltimeterSetting","synthetic":false,"types":[]}];
implementors["gauge_sys"] = [{"text":"impl Copy for FsContext","synthetic":false,"types":[]},{"text":"impl Copy for RawServiceId","synthetic":false,"types":[]},{"text":"impl Copy for ServiceId","synthetic":false,"types":[]},{"text":"impl Copy for GaugeDrawData","synthetic":false,"types":[]},{"text":"impl Copy for RawUnit","synthetic":false,"types":[]},{"text":"impl Copy for RawAircraftVariable","synthetic":false,"types":[]},{"text":"impl Copy for RawNamedVariable","synthetic":false,"types":[]}];
implementors["simconnect_sys"] = [{"text":"impl Copy for ReceiveHeader","synthetic":false,"types":[]},{"text":"impl Copy for ReceiveOpen","synthetic":false,"types":[]},{"text":"impl Copy for Version","synthetic":false,"types":[]},{"text":"impl Copy for ReceiveEvent","synthetic":false,"types":[]},{"text":"impl Copy for RawDataDefinitionId","synthetic":false,"types":[]},{"text":"impl Copy for RawObjectId","synthetic":false,"types":[]},{"text":"impl Copy for RawNotificationGroupId","synthetic":false,"types":[]},{"text":"impl Copy for RawEventId","synthetic":false,"types":[]},{"text":"impl Copy for NotificationGroupPriority","synthetic":false,"types":[]},{"text":"impl Copy for RawDataSetFlag","synthetic":false,"types":[]},{"text":"impl Copy for DataSetFlag","synthetic":false,"types":[]},{"text":"impl Copy for RawMessageType","synthetic":false,"types":[]},{"text":"impl Copy for MessageType","synthetic":false,"types":[]},{"text":"impl Copy for RawDataType","synthetic":false,"types":[]},{"text":"impl Copy for DataType","synthetic":false,"types":[]},{"text":"impl Copy for SimConnectHandle","synthetic":false,"types":[]},{"text":"impl Copy for Handle","synthetic":false,"types":[]},{"text":"impl Copy for WindowHandle","synthetic":false,"types":[]},{"text":"impl Copy for HResult","synthetic":false,"types":[]}];
implementors["wt_cj4"] = [{"text":"impl Copy for ThrottleMode","synthetic":false,"types":[]},{"text":"impl Copy for ThrottleAxis","synthetic":false,"types":[]},{"text":"impl Copy for ThrustValue","synthetic":false,"types":[]},{"text":"impl Copy for ThrottlePercent","synthetic":false,"types":[]},{"text":"impl Copy for EngineNumber","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Copy&gt; Copy for EngineData&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl Copy for FadecController","synthetic":false,"types":[]},{"text":"impl Copy for Instruments","synthetic":false,"types":[]},{"text":"impl Copy for EngineReadings","synthetic":false,"types":[]},{"text":"impl Copy for Environment","synthetic":false,"types":[]},{"text":"impl Copy for Engine","synthetic":false,"types":[]},{"text":"impl Copy for Aircraft","synthetic":false,"types":[]},{"text":"impl Copy for Snapshot","synthetic":false,"types":[]}];
implementors["wt_systems"] = [{"text":"impl&lt;In&gt; Copy for PidConfiguration&lt;In&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;In: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Ratio: Div&lt;In&gt; + Div&lt;RetainedError&lt;Time, In&gt;&gt; + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Time: Mul&lt;In&gt; + Div&lt;In&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Proportion&lt;Ratio, In&gt;: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Integral&lt;Ratio, In, Time&gt;: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Derivative&lt;Time, In&gt;: Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;In&gt; Copy for PidController&lt;In&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;In: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Ratio: Div&lt;In&gt; + Div&lt;RetainedError&lt;Time, In&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Time: Mul&lt;In&gt; + Div&lt;In&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;RetainedError&lt;Time, In&gt;: Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;In&gt; Copy for PidConfiguration&lt;In&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;In: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Ratio: Div&lt;In&gt; + Div&lt;RetainedError&lt;Time, In&gt;&gt; + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Time: Mul&lt;In&gt; + Div&lt;In&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Proportion&lt;Ratio, In&gt;: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Integral&lt;Ratio, In, Time&gt;: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Derivative&lt;Time, In&gt;: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;RetainedError&lt;Time, In&gt;: Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;In&gt; Copy for PidController&lt;In&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;In: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;Ratio: Div&lt;In&gt; + Div&lt;RetainedError&lt;Time, In&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Time: Mul&lt;In&gt; + Div&lt;In&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;RetainedError&lt;Time, In&gt;: Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Copy for PidComponents","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()